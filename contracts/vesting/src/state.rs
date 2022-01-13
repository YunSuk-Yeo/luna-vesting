use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use common::vesting::{StakingInfo, VestingSchedule};
use cosmwasm_std::Uint128;
use cw20::Denom;
use cw_storage_plus::Item;

pub const CONFIG: Item<Config> = Item::new("config");
pub const VESTING_INFO: Item<VestingInfo> = Item::new("vesting_info");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub owner_address: String,
    pub staking_enabled: bool,
    pub staking_info: Option<StakingInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct VestingInfo {
    pub vesting_denom: Denom,
    pub vesting_amount: Uint128,
    pub vesting_schedule: VestingSchedule,
    pub claimed_amount: Uint128,
}
