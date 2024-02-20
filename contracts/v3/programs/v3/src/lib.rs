use anchor_lang::prelude::*;

pub mod instructions;
pub mod math;
pub mod state;
pub mod errors;
use instructions::*;

declare_id!("4H2r9mHsejk4bRCRr1PoY626uMytjZ3WpVYaT9CtGTdr");

#[program]
pub mod v3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        tick_spacing: u16,
        initial_sqrt_price: u128,
    ) -> Result<()> {
        return instructions::initialize_pool::handler(
            ctx,
            tick_spacing,
            initial_sqrt_price,
        );
    }
}

#[derive(Accounts)]
pub struct Initialize {}
