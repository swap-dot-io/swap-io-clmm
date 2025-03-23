use std::collections::VecDeque;

use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas, SwapMode, SwapParams 
};
use solana_sdk::pubkey::Pubkey;
use spl_token_2022::{extension::StateWithExtensions, state::Mint};
use swap_io_clmm::states::{AmmConfig, PoolState, TickArrayBitmapExtension, TickArrayState};
use anyhow::Result;

use rust_decimal::Decimal;

use crate::utils::{amount_with_slippage, get_out_put_amount_fee_and_remaining_accounts, get_transfer_fee, get_transfer_inverse_fee};

pub struct SwapIoClmmAdapter {
    pool_key: Pubkey,
    pool_state: PoolState,
    token_a_decimals: u8,
    token_b_decimals: u8,
    program_id: Pubkey,
    epoch: u64,
    mint0_state: Option<StateWithExtensions<'static, Mint>>,
    mint1_state: Option<StateWithExtensions<'static, Mint>>,
    tick_arrays: VecDeque<TickArrayState>,
    amm_config: Option<AmmConfig>,
    tickarray_bitmap_extension: Option<TickArrayBitmapExtension>
    
}

impl SwapIoClmmAdapter {
    fn new(
        pool_key: Pubkey,
        pool_state: PoolState,
        program_id: Pubkey,
    ) -> Self {
        let token_a_decimals = pool_state.mint_decimals_0;
        let token_b_decimals = pool_state.mint_decimals_1;
        Self {
            pool_key,
            pool_state,
            token_a_decimals,
            token_b_decimals,
            program_id,
            epoch: 0,
            mint0_state: None,
            mint1_state: None,
            tick_arrays: VecDeque::new(),
            amm_config: None,
            tickarray_bitmap_extension: None
        }
    }
    
    // Make these getters public for testing
    pub fn token_a_decimals(&self) -> u8 {
        self.token_a_decimals
    }
    
    pub fn token_b_decimals(&self) -> u8 {
        self.token_b_decimals
    }

    pub fn pool_state(&self) -> &PoolState {
        &self.pool_state
    }
}

impl Amm for SwapIoClmmAdapter where Self: Sized {
    fn from_keyed_account(keyed_account: &KeyedAccount, _amm_context: &AmmContext) -> Result<Self> {
        let pool_key = keyed_account.key;
        let pool_data: &[u8] = keyed_account.account.data.as_ref();

        // Check if we have the 8-byte discriminator at the beginning
        if pool_data.len() < 8 {
            return Err(anyhow::anyhow!("Account data too short"));
        }
        
        // Debug information about the account data
        println!("Account data length: {}", pool_data.len());
        
        // For Anchor programs, we need to skip the 8-byte discriminator
        let data_without_discriminator = &pool_data[8..];
        println!("Data without discriminator length: {}", data_without_discriminator.len());
        
        // Check size requirements
        println!("PoolState size: {}", std::mem::size_of::<PoolState>());
        
        // Deserialize pool_state
        println!("About to unpack PoolState");
        // Use the minimal deserialization function
        let pool_state: &PoolState =
                bytemuck::from_bytes(&pool_data[8..core::mem::size_of::<PoolState>() + 8]);
        
        let program_id = keyed_account.account.owner;
        Ok(Self::new(pool_key, *pool_state, program_id))
    }
    
    fn label(&self) -> String {
        "SWAP-IO-CLMM".to_string()
    }
    
    fn program_id(&self) -> Pubkey {
        self.program_id
    }
    
    fn key(&self) -> Pubkey {
        self.pool_key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.pool_state.token_mint_0, self.pool_state.token_mint_1]
    }
    
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        todo!()
    }
    
    fn update(&mut self, _account_map: &AccountMap) -> Result<()> {
        todo!()
    }
    
    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        
            // let [user_input_account, user_output_account, amm_config_account, pool_account, tickarray_bitmap_extension_account, mint0_account, mint1_account] =
                // array_ref![rsps, 0, 7];

            // let user_input_token_data = self.user_input_account.clone().unwrap().data;
            // let user_input_state = StateWithExtensions::<Account>::unpack(&user_input_token_data)?;
            // let user_output_token_data = user_output_account.clone().unwrap().data;
            // let user_output_state =
            //     StateWithExtensions::<Account>::unpack(&user_output_token_data)?;
            // let mint0_data = mint0_account.clone().unwrap().data;
            // let mint0_state = StateWithExtensions::<Mint>::unpack(&mint0_data)?;
            // let mint1_data = mint1_account.clone().unwrap().data;
            // let mint1_state = StateWithExtensions::<Mint>::unpack(&mint1_data)?;
            // let amm_config_state = deserialize_anchor_account::<swap_io_clmm::states::AmmConfig>(
            //     amm_config_account.as_ref().unwrap(),
            // )?;
            // let pool_state = deserialize_anchor_account::<swap_io_clmm::states::PoolState>(
            //     pool_account.as_ref().unwrap(),
            // )?;
            // let tickarray_bitmap_extension =
            //     deserialize_anchor_account::<swap_io_clmm::states::TickArrayBitmapExtension>(
            //         tickarray_bitmap_extension_account.as_ref().unwrap(),
            //     )?;
            let zero_for_one = quote_params.input_mint == self.pool_state.token_mint_0
                && quote_params.output_mint == self.pool_state.token_mint_1;

            let base_in = quote_params.swap_mode == SwapMode::ExactIn;
            let amount = quote_params.amount;
            let mint0_state = self.mint0_state.as_ref().expect("Mint0State must be initialized");
            let mint1_state = self.mint1_state.as_ref().expect("Mint1State must be initialized");
            let transfer_fee = if base_in {
                if zero_for_one {
                    get_transfer_fee(mint0_state, self.epoch, amount)
                } else {
                    get_transfer_fee(mint1_state, self.epoch, amount)
                }
            } else {
                0
            };
            let amount_specified = amount.checked_sub(transfer_fee).unwrap();
            // load tick_arrays
            let mut tick_arrays = self.tick_arrays.clone();

            let sqrt_price_limit_x64 = None;
            // if limit_price.is_some() {
            //     let sqrt_price_x64 = price_to_sqrt_price_x64(
            //         limit_price.unwrap(),
            //         pool_state.mint_decimals_0,
            //         pool_state.mint_decimals_1,
            //     );
            //     sqrt_price_limit_x64 = Some(sqrt_price_x64);
            // }
            
            let (mut other_amount_threshold, fee_amount) =
                get_out_put_amount_fee_and_remaining_accounts(
                    amount_specified,
                    sqrt_price_limit_x64,
                    zero_for_one,
                    base_in,
                    self.amm_config.as_ref().expect("AmmConfig must be initialized"),
                    &self.pool_state,
                    &self.tickarray_bitmap_extension.as_ref().expect("TickArrayBitmapExtension must be initialized"),
                    &mut tick_arrays,
                )
                .unwrap();
            println!(
                "amount:{}, other_amount_threshold:{}",
                amount, other_amount_threshold
            );
            if base_in {
                // calc mint out amount with slippage
                other_amount_threshold =
                    amount_with_slippage(other_amount_threshold, 0.0, false);
            } else {
                // calc max in with slippage
                other_amount_threshold =
                    amount_with_slippage(other_amount_threshold, 0.0, true);
                // calc max in with transfer_fee
                let transfer_fee = if zero_for_one {
                    get_transfer_inverse_fee(&mint0_state, self.epoch, other_amount_threshold)
                } else {
                    get_transfer_inverse_fee(&mint1_state, self.epoch, other_amount_threshold)
                };
                other_amount_threshold += transfer_fee;
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

            let config_state = self.amm_config.as_ref().expect("AmmConfig must be initialized");
            // The trade fee, denominated in hundredths of a bip (10^-6)
            // pub trade_fee_rate: u32,
            let fee_pct: Decimal = Decimal::new(config_state.trade_fee_rate as i64, 6);

            Ok(Quote {
                fee_pct: fee_pct,
                in_amount: in_amount,
                out_amount: out_amount,
                fee_amount: fee_amount,
                fee_mint: quote_params.input_mint,
                ..Quote::default()
            })
    }
    
    fn get_swap_and_account_metas(&self, __swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        todo!()
    }
    
    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        todo!()
    }
    
    fn has_dynamic_accounts(&self) -> bool {
        false
    }
    
    fn requires_update_for_reserve_mints(&self) -> bool {
        false
    }
    
    fn supports_exact_out(&self) -> bool {
        false
    }
    
    fn get_user_setup(&self) -> Option<jupiter_amm_interface::AmmUserSetup> {
        None
    }
    
    fn unidirectional(&self) -> bool {
        false
    }
    
    fn program_dependencies(&self) -> Vec<(Pubkey, String)> {
        std::vec![]
    }
    
    fn get_accounts_len(&self) -> usize {
        32 // Default to a near whole legacy transaction to penalize no implementation
    }
    
    fn underlying_liquidities(&self) -> Option<std::collections::HashSet<Pubkey>> {
        None
    }
    
    fn is_active(&self) -> bool {
        true
    }
}