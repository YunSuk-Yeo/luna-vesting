use cosmwasm_std::{Decimal, StdError, StdResult, Uint128};
use cw20::Denom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub owner_address: String,
    pub enable_staking: bool,
    pub staking_info: Option<StakingInfo>,
    pub vesting_schedule: VestingSchedule,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ChangeOwner { new_owner: String },
    Claim { recipient: Option<String> },
    ClaimRewards { recipient: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    VestingInfo {},
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct VestingInfoResponse {
    pub owner_address: String,
    pub vesting_denom: Denom,
    pub vesting_amount: Uint128,
    pub vested_amount: Uint128,
    pub vesting_schedule: VestingSchedule,
    pub claimable_amount: Uint128,
    pub claimable_staking_rewards: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StakingInfo {
    pub bluna_token: String,
    pub hub_contract: String,
    pub reward_contract: String,
}

/// VestingSchedule is used to vest tokens
/// at regular intervals for a specific period.
/// To minimize calculation error,
/// (end_time - start_time) should be multiple of vesting_interval
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VestingSchedule {
    pub start_time: String,       // vesting start time in second unit
    pub end_time: String,         // vesting end time in second unit
    pub vesting_interval: String, // vesting interval in second unit
}

impl VestingSchedule {
    pub fn validate(&self, block_time: u64, vesting_amount: Uint128) -> StdResult<()> {
        if vesting_amount.is_zero() {
            return Err(StdError::generic_err(
                "cannot make zero token vesting account",
            ));
        }

        let start_time = self
            .start_time
            .parse::<u64>()
            .map_err(|_| StdError::generic_err("invalid start_time"))?;

        let end_time = self
            .end_time
            .parse::<u64>()
            .map_err(|_| StdError::generic_err("invalid end_time"))?;

        let vesting_interval = self
            .vesting_interval
            .parse::<u64>()
            .map_err(|_| StdError::generic_err("invalid vesting_interval"))?;

        if start_time < block_time {
            return Err(StdError::generic_err("invalid start_time"));
        }

        if end_time <= start_time {
            return Err(StdError::generic_err(
                "end_time must be bigger than start_time",
            ));
        }

        if vesting_interval == 0 {
            return Err(StdError::generic_err("vesting_interval must be non-zero"));
        }

        let time_period = end_time - start_time;
        let num_interval = time_period / vesting_interval;
        if time_period != num_interval * vesting_interval {
            return Err(StdError::generic_err(
                "(end_time - start_time) must be multiple of vesting_interval",
            ));
        }

        Ok(())
    }

    pub fn vested_amount(&self, block_time: u64, vesting_amount: Uint128) -> StdResult<Uint128> {
        let start_time = self.start_time.parse::<u64>().unwrap();
        let end_time = self.end_time.parse::<u64>().unwrap();
        let vesting_interval = self.vesting_interval.parse::<u64>().unwrap();
        if block_time <= start_time {
            return Ok(Uint128::zero());
        }

        if block_time >= end_time {
            return Ok(vesting_amount);
        }

        let passed_intervals = (block_time - start_time) / vesting_interval;
        let num_intervals = (end_time - start_time) / vesting_interval;
        let vesting_ratio = Decimal::from_ratio(1u64, num_intervals);
        Ok((vesting_ratio * vesting_amount).checked_mul(Uint128::from(passed_intervals))?)
    }
}

#[test]
fn vested_amount() {
    let schedule = VestingSchedule {
        start_time: "100".to_string(),
        end_time: "110".to_string(),
        vesting_interval: "5".to_string(),
    };

    let vesting_amount = Uint128::new(1000000u128);
    assert_eq!(
        schedule.vested_amount(100, vesting_amount).unwrap(),
        Uint128::zero()
    );
    assert_eq!(
        schedule.vested_amount(105, vesting_amount).unwrap(),
        Uint128::new(500000u128)
    );
    assert_eq!(
        schedule.vested_amount(110, vesting_amount).unwrap(),
        Uint128::new(1000000u128)
    );
    assert_eq!(
        schedule.vested_amount(115, vesting_amount).unwrap(),
        Uint128::new(1000000u128)
    );
}
