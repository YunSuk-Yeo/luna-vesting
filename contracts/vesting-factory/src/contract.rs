use common::{
    vesting::{InstantiateMsg as VestingInstantiateMsg, VestingSchedule},
    vesting_factory::{ExecuteMsg, InstantiateMsg, QueryMsg, VestingContractResponse},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg, WasmMsg,
};
use protobuf::Message;

use crate::response::MsgInstantiateContractResponse;
use crate::state::{Config, CONFIG, TMP_STORE, VESTING_CONTRACTS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    CONFIG.save(
        deps.storage,
        &Config {
            vesting_contract_code_id: msg.vesting_contract_code_id,
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateVestingContract {
            owner_address,
            enable_staking,
            vesting_schedule,
        } => create_vesting_contract(
            deps,
            env,
            info,
            owner_address,
            enable_staking,
            vesting_schedule,
        ),
    }
}

/// This will check converted bluna amount and set
/// the amount as vesting amount.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    if msg.id != 1 {
        return Err(StdError::generic_err("unauthorized"));
    }

    let res: MsgInstantiateContractResponse =
        Message::parse_from_bytes(msg.result.unwrap().data.unwrap().as_slice()).map_err(|_| {
            StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
        })?;

    let owner_address = TMP_STORE.load(deps.storage)?;
    let vesting_contract = res.get_contract_address();
    VESTING_CONTRACTS.save(
        deps.storage,
        owner_address.to_string(),
        &vesting_contract.to_string(),
    )?;
    TMP_STORE.remove(deps.storage);

    Ok(Response::new().add_attributes(vec![
        ("action", "create_vesting_contract"),
        ("owner_address", &owner_address),
        ("vesting_contract", &vesting_contract.to_string()),
    ]))
}

fn create_vesting_contract(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner_address: String,
    enable_staking: bool,
    vesting_schedule: VestingSchedule,
) -> StdResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;
    if VESTING_CONTRACTS
        .may_load(deps.storage, owner_address.to_string())?
        .is_some()
    {
        return Err(StdError::generic_err("already exist"));
    }

    TMP_STORE.save(deps.storage, &owner_address)?;

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: Some(env.contract.address.to_string()),
            code_id: config.vesting_contract_code_id,
            msg: to_binary(&VestingInstantiateMsg {
                owner_address,
                enable_staking,
                vesting_schedule,
            })?,
            funds: info.funds,
            label: "".to_string(),
        }),
        1,
    )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VestingContract { owner_address } => {
            to_binary(&vesting_contract(deps, owner_address)?)
        }
    }
}

fn vesting_contract(deps: Deps, owner_address: String) -> StdResult<VestingContractResponse> {
    let vesting_contract = VESTING_CONTRACTS.load(deps.storage, owner_address.to_string())?;

    Ok(VestingContractResponse {
        owner_address,
        vesting_contract,
    })
}
