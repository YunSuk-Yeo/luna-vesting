# Luna Vesting Contract

This project provides two contracts
* [factory](./contracts/vesting-factory): A contract to instnatiate a new vesting contract and store vesting contract address.
* [vesting](./contracts/vesting): Luna vesting contract.

A creator executes `CreateVestingAccount` function of `Factory` contract with vesting LUNA token and following inputs.
* `owner_address`: vesting claim address
* `enable_staking`: whether the vesting LUNA staked or not.
* `vesting_schedule`: vesting schedule (periodic or linear) 

 Each `CreateVestingAccount` execution will instantiate a new `Vesting` contract. The created vesting contract is registered to the Factory contract and can be queried with `owner_address` parameter.

