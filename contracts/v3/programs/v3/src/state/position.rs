use anchor_lang::prelude::*;
use super::{Pool};

#[account]
#[derive(Default)]
pub struct Position {
    pub pool: Pubkey,
    pub position_mint: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32
}

impl Position {

    pub const LEN: usize = 8 + 88;

    pub fn update(&mut self, liquidity: u128) {
        self.liquidity = liquidity;
    }

    pub fn open_position( &mut self,
        pool: &Account<Pool>,
        position_mint: Pubkey,
        tick_lower_index: i32,
        tick_upper_index: i32
    ) -> Result<()> {
        self.pool = pool.key();
        self.position_mint = position_mint;

        self.tick_lower_index = tick_lower_index;
        self.tick_upper_index = tick_upper_index;
        Ok(())
    }

}