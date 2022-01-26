## Token Vesting

This contract is to provide LUNA vesting account feature and the vesting LUNA can be staked via Anchor Protocol.
The contract will be generated per a vesting account to separate staking rewards.

### Initiate Contract

When a initiator enable staking, the withdraw address of staking rewards will be set to `owner_address`.

* disable staking
  ```json
  {
      "owner_address": "terra1~~",
      "enable_staking": false,
      "vesting_schedule": {
          "start_time": "16838388123",
          "end_time": "16838388133",
          "vesting_interval": "1", // vesting interval in second unit
      }
  }
  ```
* enable staking
  ```json
  {
      "owner_address": "terra1~~",
      "enable_staking": true,
      "vesting_schedule": {
          "start_time": "16838388123",
          "end_time": "16838388126",
          "vesting_interval": "1", // vesting interval in second unit
      }
  }
  ```

### Vesting Account Operations

* ChangeOwner - change claim privileged account address to other address, the withdraw address of staking rewards will be set to new `owner_address`.
* Claim - send newly vested token to the (`recipient` or `vesting_account`). The `claim_amount` is computed as (`vested_amount` - `claimed_amount`) and `claimed_amount` is updated to `vested_amount`.
* ClaimRewards - withdraw staking rewards to `owner_address`. This function only can be executed when `staking_enabled` is true
* Delegate - relay delegate message 
* Undelegate - relay unbond message 
* Redelegate - relay redelegate message

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ChangeOwner {
        new_owner: String,
    },
    Claim {
        recipient: Option<String>,
    },
    ClaimRewards {
        validators: Vec<String>,
    },
    Delegate {
        validator: String,
        amount: Uint128,
    },
    Undelegate {
        validator: String,
        amount: Uint128,
    },
    Redelegate {
        src_validator: String,
        dst_validator: String,
        amount: Uint128,
    },
}
```

### Deployed Contract CodeID

| columbus-5 | bombay-12 |
| ---------- | --------- |
| N/A        | 37363     |
