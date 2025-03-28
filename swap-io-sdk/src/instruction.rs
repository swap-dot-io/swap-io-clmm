// swap_io_clmm_sdk/src/instruction.rs
use crate::pool::PoolManager;
use anyhow::Result;
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

pub struct InstructionBuilder;

impl InstructionBuilder {
    pub fn build_swap_instruction(
        pool_manager: &PoolManager,
        source_mint: Pubkey,
        destination_mint: Pubkey,
        source_token_account: Pubkey,
        destination_token_account: Pubkey,
    ) -> Result<Instruction> {
        let zero_for_one: bool = source_mint == pool_manager.pool_state.token_mint_0
            && destination_mint == pool_manager.pool_state.token_mint_1;

        let (input_vault, output_vault, input_vault_mint, output_vault_mint) = if zero_for_one {
            (
                pool_manager.pool_state.token_vault_0,
                pool_manager.pool_state.token_vault_1,
                pool_manager.pool_state.token_mint_0,
                pool_manager.pool_state.token_mint_1,
            )
        } else {
            (
                pool_manager.pool_state.token_vault_1,
                pool_manager.pool_state.token_vault_0,
                pool_manager.pool_state.token_mint_1,
                pool_manager.pool_state.token_mint_0,
            )
        };

        let mut account_metas = vec![
            //amm_config
            AccountMeta::new_readonly(pool_manager.pool_state.amm_config, false),
            //pool_state
            AccountMeta::new(pool_manager.pool_key, false),
            //input_token_account
            AccountMeta::new(source_token_account, false),
            //output_token_account
            AccountMeta::new(destination_token_account, false),
            //input_vault
            AccountMeta::new(input_vault, false),
            //output_vault
            AccountMeta::new(output_vault, false),
            //observation_state
            AccountMeta::new(pool_manager.pool_state.observation_key, false),
            //token_program
            AccountMeta::new_readonly(spl_token::id(), false),
            //token_program_2022
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            //memo_program
            AccountMeta::new_readonly(spl_memo::id(), false),
            //input_vault_mint
            AccountMeta::new_readonly(input_vault_mint, false),
            //output_vault_mint
            AccountMeta::new_readonly(output_vault_mint, false),
            //tickarray_bitmap_extension
            AccountMeta::new(pool_manager.tick_array_bitmap_extension(), false),
        ];

        //if zero_for_one extend account_metas with up_tick_array_keys, else with down_tick_array_keys
        if zero_for_one {
            for key in pool_manager.get_up_tick_array_keys() {
                account_metas.push(AccountMeta::new(key, false));
            }
        } else {
            for key in pool_manager.get_down_tick_array_keys() {
                account_metas.push(AccountMeta::new(key, false));
            }
        }

        Ok(Instruction { program_id: pool_manager.program_id, accounts: account_metas, data: vec![] })
    }
}
