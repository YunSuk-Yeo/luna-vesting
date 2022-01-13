use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RewardContractExecuteMsg {
    /// Request bAsset reward withdrawal
    ClaimRewards { recipient: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RewardContractQueryMsg {
    /// Request bAsset reward amount
    AccruedRewards { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HubContractExecuteMsg {
    /// Receives `amount` in underlying coin denom from sender.
    /// Delegate `amount` equally between validators from the registry.
    /// Issue `amount` / exchange_rate for the user.
    Bond {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccruedRewardsResponse {
    pub rewards: Uint128,
}
