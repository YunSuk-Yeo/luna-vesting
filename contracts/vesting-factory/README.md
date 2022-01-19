## Vesting Contract Factory

This contract is to generate vesting contract and manage instantiated vesting contracts.

### Initiate Contract

```rust
/// refer here: https://docs.anchorprotocol.com/smart-contracts/deployed-contracts#bluna-smart-contracts
/// to anchor contract infos
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub bluna_token: String,
    pub hub_contract: String,
    pub reward_contract: String,
    pub vesting_contract_code_id: u64,
}
```

### Execute Contract
* CreateVestingContract - instantiate vesting contract and store owner-vesting contract pair info to store

When a initiator enable staking, the deposited LUNA will be converted into bLUNA via Anchor Hub Contract.
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateVestingContract {
        owner_address: String,
        enable_staking: bool,
        vesting_schedule: VestingSchedule,
    },
}
```

### Query Vesting Contract
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    VestingContract { owner_address: String },
}
```

### Deployed Contract CodeID

| columbus-5 | bombay-12 |
| ---------- | --------- |
| N/A        | 35327     |

### Deployed Contract Address

| columbus-5 | bombay-12                                   |
| ---------- | ------------------------------------------- |
| N/A        | terra1s4ykzy85dgfc6q48pj0h33nqp7t29ntg8nree |
