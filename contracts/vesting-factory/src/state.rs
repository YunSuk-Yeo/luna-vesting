use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");
pub const TMP_STORE: Item<String> = Item::new("tmp_store");
pub const VESTING_CONTRACTS: Map<String, String> = Map::new("vesting_contracts");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub vesting_contract_code_id: u64,
}
