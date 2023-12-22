use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

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

    pub fn create_pool(
        ctx: Context<CreateLiquidityPool>
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.pool_wallet_a = ctx.accounts.pool_wallet_token_a.key().clone();
        pool.pool_wallet_b = ctx.accounts.pool_wallet_token_b.key().clone();
        pool.k_constant = 4000;
        Ok(())
    }

    pub fn deposit_liquidity(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_a.amount;
        // let pool_balance = ctx.accounts.pool_wallet_token_b.amount;

        msg!("depositor balance [{}]", depositor_balance);

        let pool = ctx.accounts.pool.clone();
        let pool_balance = ctx.accounts.pool_wallet_state.amount;
        println!("pool balance [{}]", pool_balance);
        // load pool state
        // let pool = &mut ctx.accounts.application_state;

        // check provider has enough of token account a
        // This specific step is very different compared to Ethereum. In Ethereum, accounts need to first set allowances towards 
        // a specific contract (like ZeroEx, Uniswap, Curve..) before the contract is able to withdraw funds. In this other case,
        // the SafePay program can use Bob's signature to "authenticate" the `transfer()` instruction sent to the token contract.
        // let mint_of_token_being_sent_pk_a = ctx.accounts.mint_of_token_being_sent_a.key().clone();
        // let application_idx_bytes = application_idx.to_le_bytes();
    //     let binding = ctx.accounts.user_wallet_token_a.key();
    //     let inner = vec![
    //         b"state".as_ref(),
    //         binding.as_ref(),
    //         // ctx.accounts.user_wallet_token_b.key().as_ref(),
    //         // mint_of_token_being_sent_pk_a.as_ref(), 
    //         // application_idx_bytes.as_ref(),
    //     ];
    //     let outer = vec![inner.as_slice()];

    //    // check provider has enough of token account b
    //     // move lp token account a to pool token account a
    //     // Below is the actual instruction that we are going to send to the Token program.
    //     let transfer_instruction = Transfer{
    //         from: ctx.accounts.user_wallet_token_a.to_account_info(),
    //         to: ctx.accounts.pool_wallet_token_b.to_account_info(),
    //         authority: ctx.accounts.user_sending.to_account_info(),
    //     };
    //     let cpi_ctx = CpiContext::new_with_signer(
    //         ctx.accounts.token_program.to_account_info(),
    //         transfer_instruction,
    //         outer.as_slice(),
    //     );

        // The `?` at the end will cause the function to return early in case of an error.
        // This pattern is common in Rust.
        // anchor_spl::token::transfer(cpi_ctx, amount)?;

        // // Mark stage as deposited.
        // // state.stage = Stage::FundsDeposited.to_code();
        // // move lp token account b to pool token account b

        // // print balances

        // let depositor_balance = ctx.accounts.user_wallet_token_a.amount;
        // let pool_balance = ctx.accounts.pool_wallet_token_b.amount;

        msg!("deposit balance [{}]", depositor_balance);
        // println!("pool balance [{}]", pool_balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositLiquidity<'info> {
    //  Derived PDAs
    #[account(
        mut,
        seeds=[b"state".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        bump,
    )]
    pool: Account<'info, Pool>,

    #[account(mut)]
    user_wallet_token_a: Account<'info, TokenAccount>, // staker
    mint_of_token_being_sent_a: Account<'info, Mint>,   // USDC
    #[account(mut)]
    pool_wallet_token_b: Account<'info, TokenAccount>,


    #[account(
        mut,
        seeds=[b"pool_wallet_token_a".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        bump,
    )]
    pool_wallet_state: Account<'info, TokenAccount>,
    // Users and accounts in the system
    // #[account(mut)]
    // user_sending: Signer<'info>,                     // Alice
    // // user_receiving: AccountInfo<'info>,              // Bob
    // mint_of_token_being_sent_a: Account<'info, Mint>,  // USDC
    // mint_of_token_being_sent_b: Account<'info, Mint>,  // ETH
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
pub struct CreateLiquidityPool<'info> {
     // Derived PDAs
     #[account(
        init,
        payer = owner,
        seeds=[b"state".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 8 + 70 + 70
    )]
    pool: Account<'info, Pool>,

    #[account(
        init,
        payer = owner,
        seeds=[b"pool_wallet_token_a".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent_a,
        token::authority=pool,
    )]
    pool_wallet_token_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        seeds=[b"pool_wallet_token_b".as_ref(), mint_of_token_being_sent_b.key().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent_b,
        token::authority=pool,
    )]
    pool_wallet_token_b: Account<'info, TokenAccount>,

    // Users and accounts in the system
    #[account(mut)]
    owner: Signer<'info>,                               // Alice
    mint_of_token_being_sent_a: Account<'info, Mint>,   // USDC
    mint_of_token_being_sent_b: Account<'info, Mint>,   // ETH

    // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

// 1 State account instance == 1 Safe Pay instance
#[account]
#[derive(Default)]
pub struct Pool {
    
    // pool wallet token b
    pool_wallet_a: Pubkey,

    // pool wallet token a
    pool_wallet_b: Pubkey,
    
    // a * b = k
    k_constant: u64
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