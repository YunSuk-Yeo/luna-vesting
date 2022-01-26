use crate::vesting::VestingSchedule;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub vesting_contract_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateVestingContract {
        owner_address: String,
        enable_staking: bool,
        vesting_schedule: VestingSchedule,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    VestingContract { owner_address: String },
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug)]
pub struct VestingContractResponse {
    pub owner_address: String,
    pub vesting_contract: String,
}
