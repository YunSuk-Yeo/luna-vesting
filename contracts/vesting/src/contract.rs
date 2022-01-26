#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, DistributionMsg, Env, MessageInfo,
    Response, StakingMsg, StdError, StdResult, Uint128,
};

use crate::state::{Config, VestingInfo, CONFIG, VESTING_INFO};
use common::vesting::{ExecuteMsg, InstantiateMsg, QueryMsg, VestingInfoResponse};

const VESTING_DENOM: &str = "uluna";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // validate owner address
    deps.api.addr_validate(&msg.owner_address)?;

    // deposit validation
    if info.funds.len() != 1 || info.funds[0].denom != VESTING_DENOM {
        return Err(StdError::generic_err(format!(
            "only {} is allowed to be deposited",
            VESTING_DENOM
        )));
    }

    // validate vesting schedule with vesting amount
    let vesting_token = info.funds[0].clone();
    msg.vesting_schedule
        .validate(env.block.time.seconds(), vesting_token.amount)?;

    let mut messages: Vec<CosmosMsg> = vec![];
    if msg.enable_staking {
        messages.push(CosmosMsg::Distribution(
            DistributionMsg::SetWithdrawAddress {
                address: msg.owner_address.to_string(),
            },
        ));
    }

    VESTING_INFO.save(
        deps.storage,
        &VestingInfo {
            vesting_amount: vesting_token.amount,
            vesting_schedule: msg.vesting_schedule,
            claimed_amount: Uint128::zero(),
        },
    )?;

    // store config
    CONFIG.save(
        deps.storage,
        &Config {
            owner_address: msg.owner_address.to_string(),
            staking_enabled: msg.enable_staking,
        },
    )?;

    Ok(Response::new()
        .add_attributes(vec![
            ("action", "create_vesting_account"),
            ("owner_address", &msg.owner_address),
            ("vesting_amount", &vesting_token.amount.to_string()),
        ])
        .add_messages(messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::ChangeOwner { new_owner } => change_owner(deps, info, new_owner),
        ExecuteMsg::Claim { recipient } => claim(deps, env, info, recipient),
        ExecuteMsg::ClaimRewards { validators } => claim_rewards(deps, info, validators),
        ExecuteMsg::Delegate { validator, amount } => delegate(deps, info, validator, amount),
        ExecuteMsg::Undelegate { validator, amount } => undelegate(deps, info, validator, amount),
        ExecuteMsg::Redelegate {
            src_validator,
            dst_validator,
            amount,
        } => redelegate(deps, info, src_validator, dst_validator, amount),
    }
}

fn change_owner(deps: DepsMut, info: MessageInfo, new_owner: String) -> StdResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if config.owner_address != info.sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    if config.staking_enabled {
        messages.push(CosmosMsg::Distribution(
            DistributionMsg::SetWithdrawAddress {
                address: new_owner.to_string(),
            },
        ));
    }

    config.owner_address = new_owner.to_string();
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attributes(vec![
            ("action", "change_owner"),
            ("new_owner", new_owner.as_str()),
        ])
        .add_messages(messages))
}

fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> StdResult<Response> {
    let sender = info.sender;
    let recipient = recipient.unwrap_or_else(|| sender.to_string());

    // permission check
    let config: Config = CONFIG.load(deps.storage)?;
    if config.owner_address != sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut vesting_info: VestingInfo = VESTING_INFO.load(deps.storage)?;
    let vested_amount = vesting_info
        .vesting_schedule
        .vested_amount(env.block.time.seconds(), vesting_info.vesting_amount)?;
    let claimed_amount = vesting_info.claimed_amount;

    let claimable_amount = vested_amount.checked_sub(claimed_amount)?;
    if claimable_amount.is_zero() {
        return Err(StdError::generic_err("nothing to claim"));
    }

    vesting_info.claimed_amount = vested_amount;
    VESTING_INFO.save(deps.storage, &vesting_info)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient.clone(),
            amount: vec![Coin {
                denom: VESTING_DENOM.to_string(),
                amount: claimable_amount,
            }],
        }))
        .add_attributes(vec![("action", "claim"), ("recipient", recipient.as_str())])
        .add_attributes(vec![
            ("vesting_amount", &vesting_info.vesting_amount.to_string()),
            ("vested_amount", &vested_amount.to_string()),
            ("claim_amount", &claimable_amount.to_string()),
        ]))
}

fn claim_rewards(deps: DepsMut, info: MessageInfo, validators: Vec<String>) -> StdResult<Response> {
    let sender = info.sender;

    // permission check
    let config: Config = CONFIG.load(deps.storage)?;
    if config.owner_address != sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    if !config.staking_enabled {
        return Err(StdError::generic_err("staking disabled"));
    }

    Ok(Response::new()
        .add_messages(
            validators
                .iter()
                .map(|v| {
                    CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                        validator: v.to_string(),
                    })
                })
                .collect::<Vec<CosmosMsg>>(),
        )
        .add_attributes(vec![("action", "claim_rewards")]))
}

fn delegate(
    deps: DepsMut,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
) -> StdResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.owner_address != info.sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    if !config.staking_enabled {
        return Err(StdError::generic_err("staking disabled"));
    }

    Ok(Response::new().add_message(StakingMsg::Delegate {
        validator,
        amount: Coin::new(amount.u128(), VESTING_DENOM),
    }))
}

fn undelegate(
    deps: DepsMut,
    info: MessageInfo,
    validator: String,
    amount: Uint128,
) -> StdResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.owner_address != info.sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    if !config.staking_enabled {
        return Err(StdError::generic_err("staking disabled"));
    }

    Ok(Response::new().add_message(StakingMsg::Undelegate {
        validator,
        amount: Coin::new(amount.u128(), VESTING_DENOM),
    }))
}

fn redelegate(
    deps: DepsMut,
    info: MessageInfo,
    src_validator: String,
    dst_validator: String,
    amount: Uint128,
) -> StdResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;
    if config.owner_address != info.sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    if !config.staking_enabled {
        return Err(StdError::generic_err("staking disabled"));
    }

    Ok(Response::new().add_message(StakingMsg::Redelegate {
        src_validator,
        dst_validator,
        amount: Coin::new(amount.u128(), VESTING_DENOM),
    }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VestingInfo {} => to_binary(&vesting_account(deps, env)?),
    }
}

fn vesting_account(deps: Deps, env: Env) -> StdResult<VestingInfoResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    let vesting_info: VestingInfo = VESTING_INFO.load(deps.storage)?;

    let vested_amount = vesting_info
        .vesting_schedule
        .vested_amount(env.block.time.seconds(), vesting_info.vesting_amount)?;
    let claimable_amount = vested_amount.checked_sub(vesting_info.claimed_amount)?;

    Ok(VestingInfoResponse {
        owner_address: config.owner_address,
        vesting_amount: vesting_info.vesting_amount,
        vested_amount,
        vesting_schedule: vesting_info.vesting_schedule,
        claimable_amount,
    })
}
