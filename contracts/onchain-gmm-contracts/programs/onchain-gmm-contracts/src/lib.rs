use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};

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

    pub fn create_pool(
        ctx: Context<CreateLiquidityPool>
    ) -> Result<()> {
        let pool = &mut ctx.accounts.application_state;
        pool.token_a_depositors = Vec::new();
        pool.token_b_depositors = Vec::new();
        pool.token_a = ctx.accounts.pool_wallet_token_a.key().clone();
        pool.token_a = ctx.accounts.pool_wallet_token_a.key().clone();
        pool.k_constant = 4000;
        Ok(())
    }

    pub fn deposit_liquidity(ctx: Context<DepositLiquidity>, amount: u16) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(application_idx: u64, deposit_bump: u8, wallet_bump: u8)]
pub struct DepositLiquidity<'info> {
     // Derived PDAs
     #[account(
        mut,
        seeds=[b"state".as_ref(), mint_of_token_being_sent_a.key().as_ref(), mint_of_token_being_sent_b.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump,
    )]
    application_state: Account<'info, Pool>,

    #[account(mut)]
    user_wallet_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    user_wallet_token_b: Account<'info, TokenAccount>,

    // Users and accounts in the system
    #[account(mut)]
    user_sending: Signer<'info>,                     // Alice
    // user_receiving: AccountInfo<'info>,              // Bob
    mint_of_token_being_sent_a: Account<'info, Mint>,  // USDC
    mint_of_token_being_sent_b: Account<'info, Mint>,  // ETH
    // // Alice's USDC wallet that has already approved the escrow wallet
    // // #[account(
    // //     mut,
    // //     constraint=wallet_to_withdraw_from.owner == user_sending.key(),
    // //     constraint=wallet_to_withdraw_from.mint == mint_of_token_being_sent.key()
    // // )]
    // // wallet_to_withdraw_from: Account<'info, TokenAccount>,

    // // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    // rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(application_idx: u64, deposit_bump: u8, wallet_bump: u8)]
pub struct CreateLiquidityPool<'info> {
     // Derived PDAs
     #[account(
        init,
        payer = user_sending,
        seeds=[b"state".as_ref(), mint_of_token_being_sent_a.key().as_ref(), mint_of_token_being_sent_b.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump,
        space = 8 + 32 + 32 + 8 + 70 + 70
    )]
    application_state: Account<'info, Pool>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user_sending,
        seeds=[b"pool_wallet_token_a".as_ref(), mint_of_token_being_sent_a.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent_a,
        token::authority=application_state,
    )]
    pool_wallet_token_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user_sending,
        seeds=[b"pool_wallet_token_b".as_ref(), mint_of_token_being_sent_b.key().as_ref(), application_idx.to_le_bytes().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent_b,
        token::authority=application_state,
    )]
    pool_wallet_token_b: Account<'info, TokenAccount>,

    // Users and accounts in the system
    #[account(mut)]
    user_sending: Signer<'info>,                     // Alice
    user_receiving: AccountInfo<'info>,              // Bob
    mint_of_token_being_sent_a: Account<'info, Mint>,  // USDC
    mint_of_token_being_sent_b: Account<'info, Mint>,  // ETH
    // Alice's USDC wallet that has already approved the escrow wallet
    // #[account(
    //     mut,
    //     constraint=wallet_to_withdraw_from.owner == user_sending.key(),
    //     constraint=wallet_to_withdraw_from.mint == mint_of_token_being_sent.key()
    // )]
    // wallet_to_withdraw_from: Account<'info, TokenAccount>,

    // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

// 1 State account instance == 1 Safe Pay instance
#[account]
#[derive(Default)]
pub struct Pool {
    
    // tokenA in the pool
    token_a: Pubkey,
    
    // tokenB in the pool
    token_b: Pubkey,
    
    // a * b = k
    k_constant: u64,

    // token a dispositors
    token_a_depositors: Vec<Pubkey>,

    // token b dispositors
    token_b_depositors: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [
            b"pool".as_ref(),
            user.key().as_ref()
        ],
        bump,
        payer = user,
        space = 8 + 2 + 1
    )]
    pub pool: Account<'info, Pool>,

    pub system_program: Program<'info, System>
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
        space = 8 + 32 + 32 + 8 + 70 + 70
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