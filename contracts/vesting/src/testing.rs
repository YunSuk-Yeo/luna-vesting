use crate::contract::{execute, instantiate, query};
use common::vesting::{ExecuteMsg, InstantiateMsg, QueryMsg, VestingInfoResponse, VestingSchedule};

use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::{
    from_binary,
    testing::{mock_env, mock_info},
    Attribute, BankMsg, Coin, CosmosMsg, DistributionMsg, StdError, SubMsg, Timestamp, Uint128,
};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: false,
        vesting_schedule: VestingSchedule {
            start_time: "105".to_string(),
            end_time: "110".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // we can just call .unwrap() to assert this was a success
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            ("action", "create_vesting_account"),
            ("owner_address", "owner0001"),
            ("vesting_amount", "1000000"),
        ]
    );
    assert_eq!(res.messages, vec![]);
}

#[test]
fn proper_initialization_enable_staking() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: true,
        vesting_schedule: VestingSchedule {
            start_time: "105".to_string(),
            end_time: "110".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // we can just call .unwrap() to assert this was a success
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Distribution(
            DistributionMsg::SetWithdrawAddress {
                address: "owner0001".to_string(),
            }
        )),]
    );
    assert_eq!(
        res.attributes,
        vec![
            ("action", "create_vesting_account"),
            ("owner_address", "owner0001"),
            ("vesting_amount", "1000000"),
        ]
    );
}

#[test]
fn invalid_start_time_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: false,
        vesting_schedule: VestingSchedule {
            start_time: "100".to_string(),
            end_time: "100".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(105);

    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn invalid_end_time_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: false,
        vesting_schedule: VestingSchedule {
            start_time: "105".to_string(),
            end_time: "100".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn test_change_owner() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: false,
        vesting_schedule: VestingSchedule {
            start_time: "105".to_string(),
            end_time: "110".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // unauthorized
    let msg = ExecuteMsg::ChangeOwner {
        new_owner: "owner0002".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap_err();
    match res {
        StdError::GenericErr { msg, .. } => assert_eq!(msg, "unauthorized"),
        _ => panic!("DO NOT ENTER"),
    }

    let info = mock_info("owner0001", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(res.messages, vec![]);

    assert_eq!(
        from_binary::<VestingInfoResponse>(
            &query(deps.as_ref(), env, QueryMsg::VestingInfo {}).unwrap()
        )
        .unwrap(),
        VestingInfoResponse {
            owner_address: "owner0002".to_string(),
            vesting_amount: Uint128::new(1000000u128),
            vested_amount: Uint128::zero(),
            vesting_schedule: VestingSchedule {
                start_time: "105".to_string(),
                end_time: "110".to_string(),
                vesting_interval: "5".to_string(),
            },
            claimable_amount: Uint128::zero(),
        }
    );
}

#[test]
fn test_change_owner_with_staking_enable() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: true,
        vesting_schedule: VestingSchedule {
            start_time: "105".to_string(),
            end_time: "110".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // unauthorized
    let msg = ExecuteMsg::ChangeOwner {
        new_owner: "owner0002".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap_err();
    match res {
        StdError::GenericErr { msg, .. } => assert_eq!(msg, "unauthorized"),
        _ => panic!("DO NOT ENTER"),
    }

    let info = mock_info("owner0001", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Distribution(
            DistributionMsg::SetWithdrawAddress {
                address: "owner0002".to_string(),
            }
        ))]
    );

    assert_eq!(
        from_binary::<VestingInfoResponse>(
            &query(deps.as_ref(), env, QueryMsg::VestingInfo {}).unwrap()
        )
        .unwrap(),
        VestingInfoResponse {
            owner_address: "owner0002".to_string(),
            vesting_amount: Uint128::new(1000000u128),
            vested_amount: Uint128::zero(),
            vesting_schedule: VestingSchedule {
                start_time: "105".to_string(),
                end_time: "110".to_string(),
                vesting_interval: "5".to_string(),
            },
            claimable_amount: Uint128::zero(),
        }
    );
}

#[test]
fn test_claim() {
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: false,
        vesting_schedule: VestingSchedule {
            start_time: "105".to_string(),
            end_time: "110".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // make time to half claimable
    env.block.time = Timestamp::from_seconds(105);

    // valid claim
    let msg = ExecuteMsg::Claim {
        recipient: Some("addr0001".to_string()),
    };

    // permission check
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap_err();
    match res {
        StdError::GenericErr { msg, .. } => assert_eq!(msg, "unauthorized"),
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("owner0001", &[]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(BankMsg::Send {
            to_address: "addr0001".to_string(),
            amount: vec![Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(500000u128),
            }],
        }),]
    );
    assert_eq!(
        res.attributes,
        vec![
            Attribute::new("action", "claim"),
            Attribute::new("recipient", "addr0001"),
            Attribute::new("vesting_amount", "1000000"),
            Attribute::new("vested_amount", "500000"),
            Attribute::new("claim_amount", "500000"),
        ],
    );

    // query vesting account
    assert_eq!(
        from_binary::<VestingInfoResponse>(
            &query(deps.as_ref(), env.clone(), QueryMsg::VestingInfo {},).unwrap()
        )
        .unwrap(),
        VestingInfoResponse {
            owner_address: "owner0001".to_string(),
            vesting_amount: Uint128::new(1000000),
            vested_amount: Uint128::new(500000),
            vesting_schedule: VestingSchedule {
                start_time: "105".to_string(),
                end_time: "110".to_string(),
                vesting_interval: "5".to_string(),
            },
            claimable_amount: Uint128::zero(),
        }
    );

    // make time to half claimable
    env.block.time = Timestamp::from_seconds(110);

    let msg = ExecuteMsg::Claim { recipient: None };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(
        res.messages,
        vec![SubMsg::new(BankMsg::Send {
            to_address: "owner0001".to_string(),
            amount: vec![Coin {
                denom: "uluna".to_string(),
                amount: Uint128::new(500000u128),
            }],
        }),]
    );
    assert_eq!(
        res.attributes,
        vec![
            Attribute::new("action", "claim"),
            Attribute::new("recipient", "owner0001"),
            Attribute::new("vesting_amount", "1000000"),
            Attribute::new("vested_amount", "1000000"),
            Attribute::new("claim_amount", "500000"),
        ],
    );

    // query vesting account
    assert_eq!(
        from_binary::<VestingInfoResponse>(
            &query(deps.as_ref(), env, QueryMsg::VestingInfo {},).unwrap()
        )
        .unwrap(),
        VestingInfoResponse {
            owner_address: "owner0001".to_string(),
            vesting_amount: Uint128::new(1000000),
            vested_amount: Uint128::new(1000000),
            vesting_schedule: VestingSchedule {
                start_time: "105".to_string(),
                end_time: "110".to_string(),
                vesting_interval: "5".to_string(),
            },
            claimable_amount: Uint128::zero(),
        }
    );
}

#[test]
fn claim_rewards() {
    let mut deps = mock_dependencies(&[Coin {
        denom: "uusd".to_string(),
        amount: Uint128::new(300u128),
    }]);

    let msg = InstantiateMsg {
        owner_address: "owner0001".to_string(),
        enable_staking: true,
        vesting_schedule: VestingSchedule {
            start_time: "105".to_string(),
            end_time: "110".to_string(),
            vesting_interval: "5".to_string(),
        },
    };

    let info = mock_info(
        "addr0000",
        &[Coin {
            denom: "uluna".to_string(),
            amount: Uint128::new(1000000),
        }],
    );
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // make time to half claimable
    env.block.time = Timestamp::from_seconds(105);

    // query vesting account
    assert_eq!(
        from_binary::<VestingInfoResponse>(
            &query(deps.as_ref(), env.clone(), QueryMsg::VestingInfo {},).unwrap()
        )
        .unwrap(),
        VestingInfoResponse {
            owner_address: "owner0001".to_string(),
            vesting_amount: Uint128::new(1000000),
            vested_amount: Uint128::new(500000),
            vesting_schedule: VestingSchedule {
                start_time: "105".to_string(),
                end_time: "110".to_string(),
                vesting_interval: "5".to_string(),
            },
            claimable_amount: Uint128::new(500000),
        }
    );

    // valid claim
    let msg = ExecuteMsg::ClaimRewards {
        validators: vec!["validator1".to_string(), "validator2".to_string()],
    };

    // permission check
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap_err();
    match res {
        StdError::GenericErr { msg, .. } => assert_eq!(msg, "unauthorized"),
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("owner0001", &[]);
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(
        res.messages,
        vec![
            SubMsg::new(DistributionMsg::WithdrawDelegatorReward {
                validator: "validator1".to_string(),
            }),
            SubMsg::new(DistributionMsg::WithdrawDelegatorReward {
                validator: "validator2".to_string(),
            }),
        ]
    );
    assert_eq!(res.attributes, vec![("action", "claim_rewards"),],);
}
