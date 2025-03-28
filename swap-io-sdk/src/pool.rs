use std::collections::VecDeque;

use anyhow::Result;
use solana_sdk::{account::Account, pubkey::Pubkey};
use swap_io_clmm::{
    libraries::{U1024, check_current_tick_array_is_initialized, tick_array_bit_map},
    states::{
        AmmConfig, POOL_TICK_ARRAY_BITMAP_SEED, PoolState, TickArrayBitmapExtension, TickArrayState,
    },
};

use crate::utils::deserialize_anchor_account;
pub const NEIGHBORHOOD_SIZE: u8 = 5;

#[derive(Clone)]
pub struct PoolManager {
    pub epoch: u64,
    pub pool_key: Pubkey,
    pub program_id: Pubkey,
    pub pool_state: PoolState,
    pub amm_config: Option<AmmConfig>,
    pub up_tick_arrays: VecDeque<TickArrayState>,
    pub down_tick_arrays: VecDeque<TickArrayState>,
    pub tickarray_bitmap_extension: Option<TickArrayBitmapExtension>,
    pub mint0_data: Option<Vec<u8>>,
    pub mint1_data: Option<Vec<u8>>,
    pub up_tick_array_keys: Vec<Pubkey>,
    pub down_tick_array_keys: Vec<Pubkey>,
}

impl PoolManager {
    pub fn new(epoch: u64, pool_key: Pubkey, program_id: Pubkey, pool_state_account: &Account) -> Result<Self> {
        let pool_state: PoolState =
            deserialize_anchor_account::<PoolState>(pool_state_account)?;
        let mut pool_manager = PoolManager {
            epoch,
            pool_key,
            program_id,
            pool_state,
            amm_config: None,
            up_tick_arrays: VecDeque::new(),
            tickarray_bitmap_extension: None,
            down_tick_arrays: VecDeque::new(),
            mint0_data: None,
            mint1_data: None,
            up_tick_array_keys: vec![],
            down_tick_array_keys: vec![],
        };

        let (up_tick_array_keys, down_tick_array_keys) =
            match pool_manager.get_nearest_tick_arrays(NEIGHBORHOOD_SIZE) {
                Ok((up_tick_arrays, down_tick_arrays)) => {
                    (up_tick_arrays.to_vec(), down_tick_arrays.to_vec())
                }
                Err(_) => (vec![], vec![]),
            };
        pool_manager.up_tick_array_keys = up_tick_array_keys;
        pool_manager.down_tick_array_keys = down_tick_array_keys;
        Ok(pool_manager)
    }

    pub fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.pool_state.token_mint_0, self.pool_state.token_mint_1]
    }

    fn get_nearest_tick_arrays_into_direction(
        &self,
        zero_for_one: bool,
        neighbor_in_each_deirection: u8,
        mut current_vaild_tick_array_start_index: i32,
    ) -> Result<Vec<Pubkey>> {
        let mut result = vec![];
        let mut max_array_size = neighbor_in_each_deirection;
        while max_array_size != 0 {
            let next_tick_array_index = self.next_initialized_tick_array_start_index(
                zero_for_one,
                current_vaild_tick_array_start_index,
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

    pub fn next_initialized_tick_array_start_index(
        &self,
        zero_for_one: bool,
        mut last_tick_array_start_index: i32,
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
            zero_for_one,
            TickArrayState::get_array_start_index(
                self.pool_state.tick_current,
                self.pool_state.tick_spacing,
            ),
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

    fn process_direction(&self, is_zero_for_one: bool, neighbor_count: u8) -> Result<Vec<Pubkey>> {
        match self.get_first_initialized_tick_array(is_zero_for_one) {
            Ok((_, start_index)) => {
                let mut tick_arrays = self.get_nearest_tick_arrays_into_direction(
                    is_zero_for_one,
                    neighbor_count,
                    start_index,
                )?;

                // Вставляем начальный массив тиков в начало вектора
                tick_arrays.insert(0, self.tick_array(start_index));
                Ok(tick_arrays)
            }
            Err(_) => Ok(vec![]),
        }
    }
    //return up and down tick arrays
    pub fn get_nearest_tick_arrays(
        &self,
        neighbor_in_each_direction: u8,
    ) -> Result<(Vec<Pubkey>, Vec<Pubkey>)> {
        if self
            .pool_state
            .is_overflow_default_tickarray_bitmap(vec![self.pool_state.tick_current])
        {
            return Ok((vec![], vec![]));
        }
        // Получаем массивы для обоих направлений
        let up_tick_arrays = self
            .process_direction(true, neighbor_in_each_direction)
            .unwrap_or_else(|_| vec![]);
        let down_tick_arrays = self
            .process_direction(false, neighbor_in_each_direction)
            .unwrap_or_else(|_| vec![]);
        Ok((up_tick_arrays, down_tick_arrays))
    }

    pub fn get_up_tick_array_keys(&self) -> Vec<Pubkey> {
        self.up_tick_arrays
            .iter()
            .map(|tick_array| tick_array.key())
            .collect()
    }

    pub fn get_down_tick_array_keys(&self) -> Vec<Pubkey> {
        self.down_tick_arrays
            .iter()
            .map(|tick_array| tick_array.key())
            .collect()
    }

    pub fn update(&mut self, account_map: Vec<&Account>, up_ticks: Vec<Account>, down_ticks: Vec<Account>) -> Result<()> {
        self.amm_config = Some(deserialize_anchor_account::<AmmConfig>(&account_map[0])?);
        self.mint0_data = Some(account_map[1].data.clone());
        self.mint1_data = Some(account_map[2].data.clone());
        self.tickarray_bitmap_extension =
            Some(deserialize_anchor_account::<TickArrayBitmapExtension>(&account_map[3])?);
        Self::update_tick_arrays(up_ticks, &mut self.up_tick_arrays)?;
        Self::update_tick_arrays(down_ticks, &mut self.down_tick_arrays)?;
        Ok(())
    }

    fn update_tick_arrays(
        account_map: Vec<Account>,
        target_arrays: &mut VecDeque<TickArrayState>,
    ) -> Result<()> {
        target_arrays.clear();
        for account in account_map {
            target_arrays.push_back(deserialize_anchor_account::<TickArrayState>(&account)?);
        }
        Ok(())
    }
}
