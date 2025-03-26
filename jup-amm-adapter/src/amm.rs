use crate::utils::{
    amount_with_slippage, deserialize_anchor_account,
    get_out_put_amount_fee_and_remaining_accounts, get_transfer_fee, get_transfer_inverse_fee,
};
use anchor_lang::prelude::AccountMeta;
use anyhow::Result;
use jupiter_amm_interface::{
    try_get_account_data, AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas, SwapMode, SwapParams
};
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022::{extension::StateWithExtensions, state::Mint};
use std::collections::VecDeque;
use swap_io_clmm::{
    accounts::SwapSingleV2, libraries::{check_current_tick_array_is_initialized, tick_array_bit_map, U1024}, states::{
        AmmConfig, PoolState, TickArrayBitmapExtension, TickArrayState, POOL_TICK_ARRAY_BITMAP_SEED
    }
};

pub const NEIGHBORHOOD_SIZE: u8 = 5;

#[derive(Clone)]
pub struct SwapIoClmmAdapter {
    epoch: u64,
    pool_key: Pubkey,
    program_id: Pubkey,
    pool_state: PoolState,
    mint0_data: Option<Vec<u8>>,
    mint1_data: Option<Vec<u8>>,
    tick_array_keys: Vec<Pubkey>,
    amm_config: Option<AmmConfig>,
    tick_arrays: VecDeque<TickArrayState>,
    tickarray_bitmap_extension: Option<TickArrayBitmapExtension>,
}

impl SwapIoClmmAdapter {
    fn new(pool_key: Pubkey, pool_state: PoolState, program_id: Pubkey, epoch: u64) -> Self {
        let mut adapter = Self {
            epoch,
            pool_key,
            pool_state,
            program_id,
            mint0_data: None,
            mint1_data: None,
            amm_config: None,
            tick_array_keys: vec![],
            tick_arrays: VecDeque::new(),
            tickarray_bitmap_extension: None,
        };
        adapter.tick_array_keys = match adapter.get_nearest_tick_arrays(NEIGHBORHOOD_SIZE) {
            Ok(tick_arrays) => tick_arrays.iter().map(|tick_array| *tick_array).collect(),
            Err(_) => vec![],
        };
        adapter
    }

    pub fn amm_config(&self) -> Option<&AmmConfig> {
        self.amm_config.as_ref()
    }

    pub fn get_tick_array_keys(&self) -> Vec<Pubkey> {
        self.tick_array_keys.clone()
    }

    pub fn pool_state(&self) -> &PoolState {
        &self.pool_state
    }

    pub fn tick_array(&self, start_array_index: i32) -> Pubkey {
        let tickarray = Pubkey::find_program_address(
            &[
                swap_io_clmm::states::TICK_ARRAY_SEED.as_bytes(),
                self.pool_key.to_bytes().as_ref(),
                &start_array_index.to_be_bytes(),
            ],
            &self.program_id,
        )
        .0;
        tickarray
    }

    pub fn tick_array_bitmap_extension(&self) -> Pubkey {
        let tickarray_bitmap_extension = Pubkey::find_program_address(
            &[
                POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
                self.pool_key.to_bytes().as_ref(),
            ],
            &self.program_id,
        )
        .0;
        tickarray_bitmap_extension
    }

    pub fn next_initialized_tick_array_start_index(
        &self,
        mut last_tick_array_start_index: i32,
        zero_for_one: bool,
    ) -> Result<Option<i32>> {
        last_tick_array_start_index = TickArrayState::get_array_start_index(
            last_tick_array_start_index,
            self.pool_state.tick_spacing,
        );

        // Search for the next initialized tick array
        let (is_found, start_index) = tick_array_bit_map::next_initialized_tick_array_start_index(
            U1024(self.pool_state.tick_array_bitmap),
            last_tick_array_start_index,
            self.pool_state.tick_spacing,
            zero_for_one,
        );

        // If we found the next initialized tick array, return it
        if is_found {
            return Ok(Some(start_index));
        }

        // Handle the case where we didn't find the next initialized tick array
        Ok(None)
    }

    pub fn get_first_initialized_tick_array(&self, zero_for_one: bool) -> Result<(bool, i32)> {
        let (is_initialized, start_index) = check_current_tick_array_is_initialized(
            U1024(self.pool_state.tick_array_bitmap),
            self.pool_state.tick_current,
            self.pool_state.tick_spacing.into(),
        )?;
        if is_initialized {
            return Ok((true, start_index));
        }
        let next_start_index = self.next_initialized_tick_array_start_index(
            TickArrayState::get_array_start_index(
                self.pool_state.tick_current,
                self.pool_state.tick_spacing,
            ),
            zero_for_one,
        )?;
        if next_start_index.is_none() {
            // If there are no initialized tick arrays in this direction,
            // return at least the current one, even if it is not initialized
            return Ok((
                false,
                TickArrayState::get_array_start_index(
                    self.pool_state.tick_current,
                    self.pool_state.tick_spacing,
                ),
            ));
        }
        return Ok((false, next_start_index.unwrap()));
    }

    fn get_nearest_tick_arrays_into_direction(
        &self,
        neighbor_in_each_deirection: u8,
        mut current_vaild_tick_array_start_index: i32,
        zero_for_one: bool,
    ) -> Result<Vec<Pubkey>> {
        let mut result = vec![];
        let mut max_array_size = neighbor_in_each_deirection;
        while max_array_size != 0 {
            let next_tick_array_index = self.next_initialized_tick_array_start_index(
                current_vaild_tick_array_start_index,
                zero_for_one,
            )?;
            if next_tick_array_index.is_none() {
                break;
            }
            current_vaild_tick_array_start_index = next_tick_array_index.unwrap();
            result.push(self.tick_array(current_vaild_tick_array_start_index));
            max_array_size -= 1;
        }
        Ok(result)
    }

    pub fn get_nearest_tick_arrays(&self, neighbor_in_each_direction: u8) -> Result<Vec<Pubkey>> {
        let mut result = vec![];

        if self
            .pool_state
            .is_overflow_default_tickarray_bitmap(vec![self.pool_state.tick_current])
        {
            return Ok(result);
        }
        // We need to return delta tick arrays in both directions
        // Calculate current tick array index
        let (_, current_vaild_tick_array_start_index_up) =
            self.get_first_initialized_tick_array(true).unwrap();
        result.push(self.tick_array(current_vaild_tick_array_start_index_up));

        let (_, current_vaild_tick_array_start_index_down) =
            self.get_first_initialized_tick_array(false).unwrap();
        //check if the down current tick array is the same as the one in the other direction, if so, we don't need to add it again
        if current_vaild_tick_array_start_index_up != current_vaild_tick_array_start_index_down {
            result.push(self.tick_array(current_vaild_tick_array_start_index_down));
        }

        let up_tick_arrays = self.get_nearest_tick_arrays_into_direction(
            neighbor_in_each_direction,
            current_vaild_tick_array_start_index_up,
            true,
        )?;
        let down_tick_arrays = self.get_nearest_tick_arrays_into_direction(
            neighbor_in_each_direction,
            current_vaild_tick_array_start_index_down,
            false,
        )?;
        result.extend(up_tick_arrays);
        result.extend(down_tick_arrays);
        Ok(result)
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch
    }
}


impl Amm for SwapIoClmmAdapter
where
    Self: Sized,
{
    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        let pool_key = keyed_account.key;
        let pool_data: &[u8] = keyed_account.account.data.as_ref();

        // Check if we have the 8-byte discriminator at the beginning
        if pool_data.len() < 8 {
            return Err(anyhow::anyhow!("Account data too short"));
        }

        let pool_state: PoolState =
            deserialize_anchor_account::<PoolState>(&keyed_account.account)?;

        let program_id = keyed_account.account.owner;
        Ok(Self::new(
            pool_key,
            pool_state,
            program_id,
            amm_context
                .clock_ref
                .epoch
                .load(std::sync::atomic::Ordering::Relaxed),
        ))
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
        let mut result = vec![];
        let state = self.pool_state;
        result.push(state.amm_config);
        result.push(state.token_mint_0);
        result.push(state.token_mint_1);
        // TickArrayBitmapExtension
        result.push(self.tick_array_bitmap_extension());
        // TickArrays
        result.extend_from_slice(&self.tick_array_keys);
        result
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let amm_config_account = account_map
            .get(&self.pool_state.amm_config)
            .ok_or_else(|| anyhow::anyhow!("AmmConfig account not found"))?;
        self.amm_config = Some(deserialize_anchor_account::<AmmConfig>(amm_config_account)?);
        // Store the token data in the struct
        self.mint0_data =
            Some(try_get_account_data(account_map, &self.pool_state.token_mint_0)?.to_vec());
        self.mint1_data =
            Some(try_get_account_data(account_map, &self.pool_state.token_mint_1)?.to_vec());
        let tickarray_bitmap_extension_account = account_map
            .get(&self.tick_array_bitmap_extension())
            .ok_or_else(|| anyhow::anyhow!("TickArrayBitmapExtension account not found"))?;
        self.tickarray_bitmap_extension = Some(deserialize_anchor_account::<
            TickArrayBitmapExtension,
        >(tickarray_bitmap_extension_account)?);
        // Load tick arrays
        self.tick_arrays.clear();
        for tick_array in &self.tick_array_keys {
            let tick_array_account = account_map
                .get(&tick_array)
                .ok_or_else(|| anyhow::anyhow!("TickArray account not found"))?;
            self.tick_arrays
                .push_back(deserialize_anchor_account::<TickArrayState>(
                    tick_array_account,
                )?);
        }
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let mint0_data = self
            .mint0_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Mint0Data not initialized"))?;
        let mint1_data = self
            .mint1_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Mint1Data not initialized"))?;
        let amm_config = self
            .amm_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("AmmConfig not initialized"))?;
        let tickarray_bitmap_extension = self
            .tickarray_bitmap_extension
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("TickArrayBitmapExtension not initialized"))?;
        let zero_for_one = quote_params.input_mint == self.pool_state.token_mint_0
            && quote_params.output_mint == self.pool_state.token_mint_1;

        let base_in = quote_params.swap_mode == SwapMode::ExactIn;
        let amount = quote_params.amount;
        let mint0_state = StateWithExtensions::<Mint>::unpack(&mint0_data)?;
        let mint1_state = StateWithExtensions::<Mint>::unpack(&mint1_data)?;
        let transfer_fee = if base_in {
            if zero_for_one {
                get_transfer_fee(&mint0_state, self.epoch, amount)
            } else {
                get_transfer_fee(&mint1_state, self.epoch, amount)
            }
        } else {
            0
        };
        let amount_specified = amount
            .checked_sub(transfer_fee)
            .ok_or(anyhow::anyhow!("Amount underflow"))?;
        // load tick_arrays
        let mut tick_arrays = self.tick_arrays.clone();

        let sqrt_price_limit_x64 = None;

        let (mut other_amount_threshold, fee_amount) =
            get_out_put_amount_fee_and_remaining_accounts(
                amount_specified,
                sqrt_price_limit_x64,
                zero_for_one,
                base_in,
                amm_config,
                &self.pool_state,
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
                get_transfer_inverse_fee(&mint0_state, self.epoch, other_amount_threshold)
            } else {
                get_transfer_inverse_fee(&mint1_state, self.epoch, other_amount_threshold)
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
            fee_mint: quote_params.input_mint,
            ..Quote::default()
        })
    }

    fn get_swap_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> Result<SwapAndAccountMetas> {
        let SwapParams {
            token_transfer_authority,
            ..
        } = swap_params;
        let result = SwapAndAccountMetas {
            swap: Swap::RaydiumClmmV2,
            account_metas: vec![
                AccountMeta::new_readonly(*token_transfer_authority, false), 
                AccountMeta::new_readonly(self.pool_state().amm_config, false),
                AccountMeta::new(self.pool_key, false),
            ]
        };
        Ok(result)
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn has_dynamic_accounts(&self) -> bool {
        false
    }

    fn requires_update_for_reserve_mints(&self) -> bool {
        false
    }

    fn supports_exact_out(&self) -> bool {
        true
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
