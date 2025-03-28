use std::collections::VecDeque;

use crate::{
    pool::PoolManager,
    utils::{
        amount_with_slippage, get_out_put_amount_fee_and_remaining_accounts, get_transfer_fee,
        get_transfer_inverse_fee,
    },
};
use anyhow::Result;
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022::{extension::StateWithExtensions, state::Mint};
use swap_io_clmm::states::TickArrayState;

#[derive(Debug, Default, Clone, Copy)]
pub struct Quote {
    pub min_in_amount: Option<u64>,
    pub min_out_amount: Option<u64>,
    pub in_amount: u64,
    pub out_amount: u64,
    pub fee_amount: u64,
    pub fee_mint: Pubkey,
    pub fee_pct: Decimal,
}

pub struct QuoteCalculator;

impl QuoteCalculator {
    pub fn calculate_quote(
        input_mint: Pubkey,
        output_mint: Pubkey,
        base_in: bool,
        amount: u64,
        pool_manager: &PoolManager,
    ) -> Result<Quote> {
        let mint0_data = pool_manager
            .mint0_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Mint0Data not initialized"))?;
        let mint1_data = pool_manager
            .mint1_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Mint1Data not initialized"))?;
        let amm_config = pool_manager
            .amm_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("AmmConfig not initialized"))?;

        let tickarray_bitmap_extension = pool_manager
            .tickarray_bitmap_extension
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("TickArrayBitmapExtension not initialized"))?;
        let zero_for_one = input_mint == pool_manager.pool_state.token_mint_0
            && output_mint == pool_manager.pool_state.token_mint_1;

        let mint0_state = StateWithExtensions::<Mint>::unpack(&mint0_data)?;
        let mint1_state = StateWithExtensions::<Mint>::unpack(&mint1_data)?;
        let transfer_fee = if base_in {
            if zero_for_one {
                get_transfer_fee(&mint0_state, pool_manager.epoch, amount)
            } else {
                get_transfer_fee(&mint1_state, pool_manager.epoch, amount)
            }
        } else {
            0
        };
        let amount_specified = amount
            .checked_sub(transfer_fee)
            .ok_or(anyhow::anyhow!("Amount underflow"))?;
        // load tick_arrays
        let mut tick_arrays: VecDeque<TickArrayState>;
        if zero_for_one {
            tick_arrays = pool_manager.up_tick_arrays.clone();
        } else {
            tick_arrays = pool_manager.down_tick_arrays.clone();
        }

        let sqrt_price_limit_x64 = None;

        let (mut other_amount_threshold, fee_amount) =
            get_out_put_amount_fee_and_remaining_accounts(
                amount_specified,
                sqrt_price_limit_x64,
                zero_for_one,
                base_in,
                amm_config,
                &pool_manager.pool_state,
                tickarray_bitmap_extension,
                &mut tick_arrays,
            )
            .unwrap();
        if base_in {
            // calc mint out amount with slippage
            other_amount_threshold = amount_with_slippage(other_amount_threshold, 0.0, false);
        } else {
            // calc max in with slippage
            other_amount_threshold = amount_with_slippage(other_amount_threshold, 0.0, true);
            // calc max in with transfer_fee
            let transfer_fee = if zero_for_one {
                get_transfer_inverse_fee(&mint0_state, pool_manager.epoch, other_amount_threshold)
            } else {
                get_transfer_inverse_fee(&mint1_state, pool_manager.epoch, other_amount_threshold)
            };
            other_amount_threshold = other_amount_threshold
                .checked_add(transfer_fee)
                .ok_or(anyhow::anyhow!("Amount overflow"))?;
        }
        let in_amount = if base_in {
            amount
        } else {
            other_amount_threshold
        };

        let out_amount = if base_in {
            other_amount_threshold
        } else {
            amount
        };

        // The trade fee, denominated in hundredths of a bip (10^-6)
        // pub trade_fee_rate: u32,
        let fee_pct: Decimal = Decimal::new(amm_config.trade_fee_rate as i64, 6);

        Ok(Quote {
            fee_pct: fee_pct,
            in_amount: in_amount,
            out_amount: out_amount,
            fee_amount: fee_amount,
            fee_mint: input_mint,
            ..Quote::default()
        })
    }
}
