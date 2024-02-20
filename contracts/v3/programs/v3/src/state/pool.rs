use crate::{
    math::{
        tick_index_from_sqrt_price, MAX_FEE_RATE, MAX_PROTOCOL_FEE_RATE, MAX_SQRT_PRICE_X64,
        MIN_SQRT_PRICE_X64,
    },
};
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Pool {
    pub tick_spacing: u16, // 2
    pub liquidity: u128, // 16 / 20
    pub sqrt_price: u128,        // 16 /36
    pub tick_current_index: i32, // 4 /40
    pub protocol_fee_owed_a: u64, // 8 /48
    pub protocol_fee_owed_b: u64, // 8/56

    pub token_mint_a: Pubkey,  // 32 /88
    pub token_vault_a: Pubkey, // 32 /120
    pub reward_last_updated_timestamp: u64, // 8 /128
    pub token_mint_b: Pubkey,  // 32 /160
    pub token_vault_b: Pubkey, // 32 /193
}

impl Pool {
    pub const LEN: usize = 8 + 193;

    pub fn initialize(
        &mut self,
        tick_spacing: u16,
        sqrt_price: u128,
        token_mint_a: Pubkey,
        token_vault_a: Pubkey,
        token_mint_b: Pubkey,
        token_vault_b: Pubkey,
    ) -> Result<()> {

        self.tick_spacing = tick_spacing;

        self.liquidity = 0;
        self.sqrt_price = sqrt_price;
        self.tick_current_index = tick_index_from_sqrt_price(&sqrt_price);

        self.protocol_fee_owed_a = 0;
        self.protocol_fee_owed_b = 0;

        self.token_mint_a = token_mint_a;
        self.token_vault_a = token_vault_a;

        self.token_mint_b = token_mint_b;
        self.token_vault_b = token_vault_b;

        Ok(())
    }

}