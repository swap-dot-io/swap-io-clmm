use std::collections::VecDeque;

use crate::utils::deserialize_anchor_account;
use anyhow::Result;
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas,
    SwapParams, try_get_account_data,
};
use solana_sdk::pubkey::Pubkey;
use swap_io_clmm::states::{AmmConfig, PoolState, TickArrayBitmapExtension, TickArrayState};
use swap_io_sdk::{instruction::InstructionBuilder, pool::PoolManager, quote::QuoteCalculator};

pub const NEIGHBORHOOD_SIZE: u8 = 5;

#[derive(Clone)]
pub struct SwapIoClmmAdapter {
    pool_manager: PoolManager,
    mint0_data: Option<Vec<u8>>,
    mint1_data: Option<Vec<u8>>,
    up_tick_array_keys: Vec<Pubkey>,
    down_tick_array_keys: Vec<Pubkey>,
}

impl SwapIoClmmAdapter {
    fn new(pool_key: Pubkey, pool_state: PoolState, program_id: Pubkey, epoch: u64) -> Self {
        let pool_manager = PoolManager::new(epoch, pool_key, program_id, pool_state);

        let (up_tick_array_keys, down_tick_array_keys) =
            match pool_manager.get_nearest_tick_arrays(NEIGHBORHOOD_SIZE) {
                Ok((up_tick_arrays, down_tick_arrays)) => {
                    (up_tick_arrays.to_vec(), down_tick_arrays.to_vec())
                }
                Err(_) => (vec![], vec![]),
            };
        Self {
            pool_manager,
            mint0_data: None,
            mint1_data: None,
            up_tick_array_keys,
            down_tick_array_keys,
        }
    }

    pub fn amm_config(&self) -> Option<&AmmConfig> {
        self.pool_manager.amm_config.as_ref()
    }

    pub fn get_up_tick_array_keys(&self) -> &Vec<Pubkey> {
        &self.up_tick_array_keys
    }
    pub fn get_down_tick_array_keys(&self) -> &Vec<Pubkey> {
        &self.down_tick_array_keys
    }

    pub fn pool_manager(&self) -> &PoolManager {
        &self.pool_manager
    }

    fn update_tick_arrays(
        account_map: &AccountMap,
        tick_array_keys: &[Pubkey],
        target_arrays: &mut VecDeque<TickArrayState>,
    ) -> Result<()> {
        target_arrays.clear();
        for tick_array in tick_array_keys {
            let tick_array_account = account_map
                .get(tick_array)
                .ok_or_else(|| anyhow::anyhow!("TickArray account not found"))?;
            target_arrays.push_back(deserialize_anchor_account::<TickArrayState>(
                tick_array_account,
            )?);
        }
        Ok(())
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
        self.pool_manager.program_id
    }

    fn key(&self) -> Pubkey {
        self.pool_manager.pool_key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        self.pool_manager.get_reserve_mints()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        let mut result = vec![];
        let state = self.pool_manager.pool_state;
        result.push(state.amm_config);
        result.push(state.token_mint_0);
        result.push(state.token_mint_1);
        // TickArrayBitmapExtension
        result.push(self.pool_manager.tick_array_bitmap_extension());
        // TickArrays
        result.extend_from_slice(&self.up_tick_array_keys);
        result.extend_from_slice(&self.down_tick_array_keys);
        result
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let amm_config_account = account_map
            .get(&self.pool_manager.pool_state.amm_config)
            .ok_or_else(|| anyhow::anyhow!("AmmConfig account not found"))?;
        self.pool_manager.amm_config =
            Some(deserialize_anchor_account::<AmmConfig>(amm_config_account)?);
        // Store the token data in the struct
        self.mint0_data = Some(
            try_get_account_data(account_map, &self.pool_manager.pool_state.token_mint_0)?.to_vec(),
        );
        self.mint1_data = Some(
            try_get_account_data(account_map, &self.pool_manager.pool_state.token_mint_1)?.to_vec(),
        );
        let tickarray_bitmap_extension_account = account_map
            .get(&self.pool_manager.tick_array_bitmap_extension())
            .ok_or_else(|| anyhow::anyhow!("TickArrayBitmapExtension account not found"))?;
        self.pool_manager.tickarray_bitmap_extension =
            Some(deserialize_anchor_account::<TickArrayBitmapExtension>(
                tickarray_bitmap_extension_account,
            )?);
        // Load tick arrays
        // Load tick arrays
        Self::update_tick_arrays(account_map, &self.up_tick_array_keys, &mut self.pool_manager.up_tick_arrays)?;
        Self::update_tick_arrays(account_map, &self.down_tick_array_keys, &mut self.pool_manager.down_tick_arrays)?;
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
        QuoteCalculator::calculate_quote(quote_params, &self.pool_manager, mint0_data, mint1_data)
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        InstructionBuilder::build_swap_instruction(&self.pool_manager, swap_params)
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
        let base_acounts = 13; //with signer
        let tick_arrsy_bitmap_extension = 1;
        let tick_array_accounts = NEIGHBORHOOD_SIZE;
        base_acounts + tick_arrsy_bitmap_extension + tick_array_accounts as usize
    }

    fn underlying_liquidities(&self) -> Option<std::collections::HashSet<Pubkey>> {
        None
    }

    fn is_active(&self) -> bool {
        true
    }
}
