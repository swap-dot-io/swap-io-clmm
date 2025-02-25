# swap.io CLMM Bug Bounty Program

The full bug bounty program for swap.io CLMM is now hosted on Hackenproof. You can find all the details at:  
https://dashboard.hackenproof.com/manager/companies/swap-dot-io/swap-dot-io-smart-contracts/program_info

## Rewards by Threat Level

Rewards are distributed according to the impact of the vulnerability based on the Hackenproof Vulnerability Severity Classification System. This is a simplified 5-level scale, focusing on the impact of the vulnerability reported.

| Severity | Bounty                         |
| -------- | ------------------------------ |
| Critical | USD 15,000 to USD 100,000      |
| High     | USD 10,000                     |
| Medium   | USD 3,000                      |

All bug reports must include a Proof of Concept (PoC) demonstrating how the vulnerability can be exploited to impact an asset in-scope to be eligible for a reward. Critical and High severity bug reports should also include a suggestion for a fix. Explanations and statements without code are not accepted as PoC.

For critical smart contract bug reports, rewards will be further capped at 10% of direct funds at risk if the vulnerability is exploited. However, there is a minimum reward of USD 15,000.

Bugs in auxiliary libraries or code outside of the smart contracts will be assessed on a case-by-case basis.

## Report Submission

All bugs should be submitted via Hackenproof using the link provided above. Please include a detailed description of the attack vector and a Proof of Concept for high- and critical-severity reports. Once submitted, you will receive a response with additional questions or next steps within 3 business days.

- **First Response:** 3 business days  
- **Triage Time:** 3 business days  
- **Reward Time:** 3 business days  
- **Resolution Time:** 14 days

## Payout Information

Payouts are handled directly by the swap.io team and are denominated in USD. Payouts can be made in RAY, SOL, or USDC.

## Out of Scope & Rules

The following vulnerabilities are excluded from rewards under this bug bounty program:

- Attacks that the reporter has already exploited themselves, leading to damage.
- Attacks requiring access to leaked keys/credentials.
- Attacks requiring access to privileged addresses (governance, strategist).
- Incorrect data supplied by third party oracles (excluding oracle manipulation/flash loan attacks).
- Basic economic governance attacks (e.g. 51% attack).
- Lack of liquidity.
- Best practice critiques.
- Sybil attacks.
- Centralization risks.
- Any UI bugs.
- Bugs in the core Solana runtime (please submit these to [Solana's bug bounty program](https://github.com/solana-labs/solana/security/policy)).
- Vulnerabilities that require a validator to execute.
- Vulnerabilities requiring access to privileged keys/credentials.
- MEV vectors the team is already aware of.
- The CLMM contract emits trading fee and farming yield tokens to LPs. If tokens from the vault or fees were drained by an attacker, users would not be able to claim yield and transactions would fail. This is by design and not considered a vulnerability.

## Concentrated Liquidity Assets in Scope

The following targets are within the scope of this bug bounty program:

| Target                                                                                                                      | Type                                       |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| [swap.io-clmm: collect_fund_fee](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/collect_fund_fee.rs)         | Smart Contract - collect_fund_fee          |
| [swap.io-clmm: collect_protocol_fee](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/collect_protocol_fee.rs)     | Smart Contract - collect_protocol_fee      |
| [swap.io-clmm: create_operation_account](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/create_operation_account.rs) | Smart Contract - create_operation_account  |
| [swap.io-clmm: admin/mod](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/mod.rs)                      | Smart Contract - admin/mod                 |
| [swap.io-clmm: transfer_reward_owner](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/transfer_reward_owner.rs)    | Smart Contract - transfer_reward_owner     |
| [swap.io-clmm: update_amm_config](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/update_amm_config.rs)        | Smart Contract - update_amm_config         |
| [swap.io-clmm: update_operation_account](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/update_operation_account.rs) | Smart Contract - update_operation_account  |
| [swap.io-clmm: update_pool_status](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/update_pool_status.rs)       | Smart Contract - update_pool_status        |
| [swap.io-clmm: close_position](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/close_position.rs)                 | Smart Contract - close_position            |
| [swap.io-clmm: collect_remaining_rewards](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/collect_remaining_rewards.rs)      | Smart Contract - collect_remaining_rewards |
| [swap.io-clmm: create_pool](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/create_pool.rs)                    | Smart Contract - create_pool               |
| [swap.io-clmm: decrease_liquidity](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/decrease_liquidity.rs)             | Smart Contract - decrease_liquidity        |
| [swap.io-clmm: increase_liquidity](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/increase_liquidity.rs)             | Smart Contract - increase_liquidity        |
| [swap.io-clmm: initialize_reward](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/initialize_reward.rs)              | Smart Contract - initialize_reward         |
| [swap.io-clmm: instructions/mod](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/mod.rs)                            | Smart Contract - instructions/mod          |
| [swap.io-clmm: open_position](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/open_position.rs)                  | Smart Contract - open_position             |
| [swap.io-clmm: set_reward_params](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/set_reward_params.rs)              | Smart Contract - set_reward_params         |
| [swap.io-clmm: swap](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/swap.rs)                           | Smart Contract - swap                      |
| [swap.io-clmm: swap_router_base_in](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/swap_router_base_in.rs)            | Smart Contract - swap_router_base_in       |
| [swap.io-clmm: update_reward_info](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/update_reward_info.rs)             | Smart Contract - update_reward_info        |
| [swap.io-clmm: big_num](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/big_num.rs)                           | Smart Contract - big_num                   |
| [swap.io-clmm: fixed_point](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/fixed_point_64.rs)                    | Smart Contract - fixed_point               |
| [swap.io-clmm: full_math](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/full_math.rs)                         | Smart Contract - full_math                 |
| [swap.io-clmm: liquidity_math](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/liquidity_math.rs)                    | Smart Contract - liquidity_math            |
| [swap.io-clmm: libraries/mod](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/mod.rs)                               | Smart Contract - libraries/mod             |
| [swap.io-clmm: sqrt_price_math](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/sqrt_price_math.rs)                   | Smart Contract - sqrt_price_math           |
| [swap.io-clmm: swap_math](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/swap_math.rs)                         | Smart Contract - swap_math                 |
| [swap.io-clmm: tick_array_bit_map](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/tick_array_bit_map.rs)                | Smart Contract - tick_array_bit_map        |
| [swap.io-clmm: tick_math](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/tick_math.rs)                         | Smart Contract - tick_math                 |
| [swap.io-clmm: unsafe_math](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/unsafe_math.rs)                       | Smart Contract - unsafe_math               |
| [swap.io-clmm: config](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/config.rs)                               | Smart Contract - config                    |
| [swap.io-clmm: states/mod](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/mod.rs)                                  | Smart Contract - states/mod                |
| [swap.io-clmm: operation_account](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/operation_account.rs)                    | Smart Contract - operation_account         |
| [swap.io-clmm: oracle](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/oracle.rs)                               | Smart Contract - oracle                    |
| [swap.io-clmm: personal_position](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/personal_position.rs)                    | Smart Contract - personal_position         |
| [swap.io-clmm: pool](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/pool.rs)                                 | Smart Contract - pool                      |
| [swap.io-clmm: protocol_position](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/protocol_position.rs)                    | Smart Contract - protocol_position         |
| [swap.io-clmm: tick_array](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/tick_array.rs)                           | Smart Contract - tick_array                |
| [swap.io-clmm: access_control](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/util/access_control.rs)                         | Smart Contract - access_control            |
| [swap.io-clmm: util/mod](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/util/mod.rs)                                    | Smart Contract - util/mod                  |
| [swap.io-clmm: system](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/util/system.rs)                                 | Smart Contract - system                    |
| [swap.io-clmm: token](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/util/token.rs)                                  | Smart Contract - token                     |
| [swap.io-clmm: error](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/error.rs)                                       | Smart Contract - error                     |
| [swap.io-clmm: lib](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/lib.rs)                                         | Smart Contract - lib                       |

## Additional Information

Documentation and instructions for creating a PoC can be found here:  
[Original Raydium CLMM Developer Documentation](https://github.com/raydium-io/raydium-docs/blob/master/dev-resources/raydium-clmm-dev-doc.pdf)
