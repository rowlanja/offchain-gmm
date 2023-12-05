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

// 1 State account instance == 1 Safe Pay instance
#[account]
#[derive(Default)]
pub struct State {

    // A primary key that allows us to derive other important accounts
    idx: u64,
    
    // Alice
    user_sending: Pubkey,

    // Bob
    user_receiving: Pubkey,

    // The Mint of the token that Alice wants to send to Bob
    mint_of_token_being_sent: Pubkey,

    // The escrow wallet
    escrow_wallet: Pubkey,

    // The amount of tokens Alice wants to send to Bob
    amount_tokens: u64,

    // An enumm that is to represent some kind of state machine
    stage: u8,
}

#[derive(Accounts)]
#[instruction(application_idx: u64, state_bump: u8, wallet_bump: u8)]
pub struct InitializeNewGrant<'info> {
    

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