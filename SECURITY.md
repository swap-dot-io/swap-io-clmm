# swap.io CLMM Audit

The Swap.io CLMM smart contract is a fork of the Raydium CLMM and was initially audited by the Raydium team. You can view the original audit report [here](https://github.com/raydium-io/raydium-docs/blob/master/audit/OtterSec%20Q3%202022/Raydium%20concentrated%20liquidity%20(CLMM)%20program.pdf).

In addition to the original audit, we have conducted an independent audit of our modifications. The audit was performed by Zenith Security and the report is available [here](https://github.com/swap-dot-io/swap-io/blob/main/audit/swap-io-clmm/2025-Zenith/Swap.io-CLMM-Zenith-Audit-Report-April-2025.pdf).

The modifications made to the Raydium CLMM codebase are minor and were assessed to introduce no significant security risks.

# swap.io CLMM Bug Bounty Program

The full bug bounty program for swap.io CLMM is now hosted on Hackenproof. You can find all the details at:  
https://hackenproof.com/programs/swap-dot-io-smart-contracts

## 💰 Rewards by Threat Level

Rewards are distributed according to the impact of the vulnerability based on the Hackenproof Vulnerability Severity Classification System. This is a simplified 5-level scale, focusing on the impact of the vulnerability reported.

| Severity | Bounty                         |
| -------- | ------------------------------ |
| Critical | USD 15,000 to USD 100,000      |
| High     | USD 10,000                     |
| Medium   | USD 3,000                      |

All bug reports must include a Proof of Concept (PoC) demonstrating how the vulnerability can be exploited to impact an asset in-scope to be eligible for a reward. Critical and High severity bug reports should also include a suggestion for a fix. Explanations and statements without code are not accepted as PoC.

For critical smart contract bug reports, rewards will be further capped at 10% of direct funds at risk if the vulnerability is exploited. However, there is a minimum reward of USD 15,000.

Bugs in auxiliary libraries or code outside of the smart contracts will be assessed on a case-by-case basis.

## 📢 Report Submission

All bugs should be submitted via Hackenproof using the link provided above. Please include a detailed description of the attack vector and a Proof of Concept for high- and critical-severity reports. Once submitted, you will receive a response with additional questions or next steps within 3 business days.

- **First Response:** 3 business days  
- **Triage Time:** 3 business days  
- **Reward Time:** 3 business days  
- **Resolution Time:** 14 days

## ℹ️ Payout Information

Rewards are administered and paid out by [Hackenproof](https://hackenproof.com/programs/swap-dot-io-smart-contracts). All payouts are denominated in USD, with rewards issued in USDC or SOL on the Solana network. Hackenproof reviews and confirms each reported bug before processing the payout.

## 🚫 Out of Scope & Rules

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
- Bugs in the core Solana runtime (please submit these to [Solana's bug bounty program](https://github.com/anza-xyz/agave/security).
- Vulnerabilities that require a validator to execute.
- Vulnerabilities requiring access to privileged keys/credentials.
- MEV vectors the team is already aware of.
- The CLMM contract emits trading fee and farming yield tokens to LPs. If tokens from the vault or fees were drained by an attacker, users would not be able to claim yield and transactions would fail. This is by design and not considered a vulnerability.
- Bugs previously fixed by the Raydium team.

Please refer to our [Hackenproof profile](https://hackenproof.com/programs/swap-dot-io-smart-contracts) for the latest list of the out of scope vulnerabilities.

## 🗒 Concentrated Liquidity Assets in Scope

The following targets are within the scope of this bug bounty program:

| File Path                                                                                                                   | Type                                       |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| [src/instructions/admin/collect_fund_fee.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/collect_fund_fee.rs)         | Smart Contract - collect_fund_fee          |
| [src/instructions/admin/collect_protocol_fee.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/collect_protocol_fee.rs)     | Smart Contract - collect_protocol_fee      |
| [src/instructions/admin/create_operation_account.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/create_operation_account.rs) | Smart Contract - create_operation_account  |
| [src/instructions/admin/mod.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/mod.rs)                      | Smart Contract - admin/mod                 |
| [src/instructions/admin/transfer_reward_owner.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/transfer_reward_owner.rs)    | Smart Contract - transfer_reward_owner     |
| [src/instructions/admin/update_amm_config.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/update_amm_config.rs)        | Smart Contract - update_amm_config         |
| [src/instructions/admin/update_operation_account.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/update_operation_account.rs) | Smart Contract - update_operation_account  |
| [src/instructions/admin/update_pool_status.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/admin/update_pool_status.rs)       | Smart Contract - update_pool_status        |
| [src/instructions/close_position.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/close_position.rs)                 | Smart Contract - close_position            |
| [src/instructions/collect_remaining_rewards.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/collect_remaining_rewards.rs)      | Smart Contract - collect_remaining_rewards |
| [src/instructions/create_pool.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/create_pool.rs)                    | Smart Contract - create_pool               |
| [src/instructions/decrease_liquidity.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/decrease_liquidity.rs)             | Smart Contract - decrease_liquidity        |
| [src/instructions/increase_liquidity.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/increase_liquidity.rs)             | Smart Contract - increase_liquidity        |
| [src/instructions/initialize_reward.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/initialize_reward.rs)              | Smart Contract - initialize_reward         |
| [src/instructions/mod.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/mod.rs)                            | Smart Contract - instructions/mod          |
| [src/instructions/open_position.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/open_position.rs)                  | Smart Contract - open_position             |
| [src/instructions/set_reward_params.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/set_reward_params.rs)              | Smart Contract - set_reward_params         |
| [src/instructions/swap.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/swap.rs)                           | Smart Contract - swap                      |
| [src/instructions/swap_router_base_in.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/swap_router_base_in.rs)            | Smart Contract - swap_router_base_in       |
| [src/instructions/update_reward_info.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/instructions/update_reward_info.rs)             | Smart Contract - update_reward_info        |
| [src/libraries/big_num.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/big_num.rs)                           | Smart Contract - big_num                   |
| [src/libraries/fixed_point_64.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/fixed_point_64.rs)                    | Smart Contract - fixed_point               |
| [src/libraries/full_math.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/full_math.rs)                         | Smart Contract - full_math                 |
| [src/libraries/liquidity_math.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/liquidity_math.rs)                    | Smart Contract - liquidity_math            |
| [src/libraries/mod.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/mod.rs)                               | Smart Contract - libraries/mod             |
| [src/libraries/sqrt_price_math.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/sqrt_price_math.rs)                   | Smart Contract - sqrt_price_math           |
| [src/libraries/swap_math.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/swap_math.rs)                         | Smart Contract - swap_math                 |
| [src/libraries/tick_array_bit_map.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/tick_array_bit_map.rs)                | Smart Contract - tick_array_bit_map        |
| [src/libraries/tick_math.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/tick_math.rs)                         | Smart Contract - tick_math                 |
| [src/libraries/unsafe_math.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/libraries/unsafe_math.rs)                       | Smart Contract - unsafe_math               |
| [src/states/config.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/config.rs)                               | Smart Contract - config                    |
| [src/states/mod.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/mod.rs)                                  | Smart Contract - states/mod                |
| [src/states/operation_account.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/operation_account.rs)                    | Smart Contract - operation_account         |
| [src/states/oracle.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/oracle.rs)                               | Smart Contract - oracle                    |
| [src/states/personal_position.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/personal_position.rs)                    | Smart Contract - personal_position         |
| [src/states/pool.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/pool.rs)                                 | Smart Contract - pool                      |
| [src/states/protocol_position.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/protocol_position.rs)                    | Smart Contract - protocol_position         |
| [src/states/tick_array.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/states/tick_array.rs)                           | Smart Contract - tick_array                |
| [src/util/mod.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/util/mod.rs)                                    | Smart Contract - util/mod                  |
| [src/util/system.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/util/system.rs)                                 | Smart Contract - system                    |
| [src/util/token.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/util/token.rs)                                  | Smart Contract - token                     |
| [src/error.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/error.rs)                                       | Smart Contract - error                     |
| [src/lib.rs](https://github.com/swap-dot-io/swap-io-clmm/blob/master/programs/amm/src/lib.rs)                                         | Smart Contract - lib                       |


## ➕ Additional Information

Documentation and instructions for creating a PoC can be found here:  
[Original Raydium CLMM Developer Documentation](https://github.com/raydium-io/raydium-docs/blob/master/dev-resources/raydium-clmm-dev-doc.pdf)
