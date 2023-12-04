use anchor_lang::prelude::*;

declare_id!("DJmR54jYwYvzAfFKCFrdpg5njsMyeAPyAEqt8usLkUE7");

#[program]
pub mod onchain_gmm_contracts {
    use super::*;

    pub fn initialize(ctx: Context<CreateDeposit>) -> Result<()> {
        let deposit = &mut ctx.accounts.deposit;
        deposit.amount = 0;
        deposit.bump = ctx.bumps.deposit;
        Ok(())
    }

    pub fn deposit(ctx: Context<UpdateDeposit>, amount: u16) -> Result<()> {
        ctx.accounts.deposit.amount = amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateDeposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [
            b"deposit".as_ref(),
            user.key().as_ref()
        ],
        bump,
        payer = user,
        space = 8 + 2 + 1
    )]
    pub deposit: Account<'info, Deposit>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateDeposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"deposit".as_ref(),
            user.key().as_ref()
        ],
        bump = deposit.bump,
    )]
    pub deposit: Account<'info, Deposit>,
}

#[account]
pub struct Deposit {
    amount: u16,
    bump: u8
}