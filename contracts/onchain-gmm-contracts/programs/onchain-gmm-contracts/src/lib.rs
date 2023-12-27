use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};

declare_id!("DJmR54jYwYvzAfFKCFrdpg5njsMyeAPyAEqt8usLkUE7");

#[program]
pub mod onchain_gmm_contracts {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreateLiquidityPool>
    ) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_a.amount;

        msg!("depositors balance [{}]", depositor_balance);

        let pool_balance = ctx.accounts.pool_wallet_token_a.amount;

        msg!("pools balance [{}]", pool_balance);

        let mint_of_token_being_sent_pk_a = ctx.accounts.mint_of_token_being_sent_a.key().clone();
        let binding = ctx.accounts.user_wallet_token_a.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        // check provider has enough of token account b
        // move lp token account a to pool token account a
        // Below is the actual instruction that we are going to send to the Token program.
        let transfer_instruction = Transfer{
            from: ctx.accounts.user_wallet_token_a.to_account_info(),
            to: ctx.accounts.pool_wallet_token_a.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        anchor_spl::token::transfer(cpi_ctx, 100)?;

        // Time to save the deposit in PDA 

        Ok(())
    }

    pub fn deposit_liquidity(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_a.amount;
        // let pool_balance = ctx.accounts.pool_wallet_token_b.amount;

        msg!("depositor balance [{}]", depositor_balance);

        let pool = ctx.accounts.pool.clone();
        let pool_token_a_balance = ctx.accounts.pool_wallet_token_a.amount;
        let pool_token_b_balance = ctx.accounts.pool_wallet_token_b.amount;
        println!("pool balance [{}]", pool_token_a_balance);
        println!("pool balance [{}]", pool_token_b_balance);
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

    #[account(
        mut,
        seeds=[b"pool_wallet_token_a".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        bump
    )]
    pool_wallet_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"pool_wallet_token_b".as_ref(), mint_of_token_being_sent_b.key().as_ref()],
        bump
    )]
    pool_wallet_token_b: Account<'info, TokenAccount>,
    
    #[account(mut)]
    depositor: Signer<'info>,                     // Alice
   
    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(
        mut,
        constraint=user_wallet_token_a.owner == depositor.key(),
        constraint=user_wallet_token_a.mint == mint_of_token_being_sent_a.key()
    )]
    user_wallet_token_a: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(
            mut,
            constraint=user_wallet_token_b.owner == depositor.key(),
            constraint=user_wallet_token_b.mint == mint_of_token_being_sent_b.key()
    )]
    user_wallet_token_b: Account<'info, TokenAccount>,

    mint_of_token_being_sent_a: Account<'info, Mint>,   // USDC
    mint_of_token_being_sent_b: Account<'info, Mint>,   // ETH


    // // Application level accounts
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    // rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1, seeds = [b"user-stats", user.key().as_ref()], bump
    )]
    pub pool_state: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        seeds=[b"pool_wallet_token_a".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent_a,
        token::authority=pool_state,
    )]
    pub pool_wallet_token_a: Account<'info, TokenAccount>,

    // #[account(
    //     init,
    //     payer = owner,
    //     seeds=[b"pool_wallet_token_b".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
    //     bump,
    //     token::mint=mint_of_token_being_sent_b,
    //     token::authority=pool,
    // )]
    // pool_wallet_token_b: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds=[b"user_stake".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        space = 8 + 2 + 32 + 32 + 8,
        bump,
    )]
    pub stake_record: Account<'info, Deposit>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_a: Account<'info, TokenAccount>,

    // // Alice's USDC wallet that has already approved the escrow wallet
    // #[account(
    //         mut,
    //         constraint=user_wallet_token_b.owner == owner.key(),
    //         constraint=user_wallet_token_b.mint == mint_of_token_being_sent_b.key()
    // )]
    // user_wallet_token_b: Account<'info, TokenAccount>,

    pub mint_of_token_being_sent_a: Account<'info, Mint>,   // USDC
    // mint_of_token_being_sent_b: Account<'info, Mint>,   // ETH

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

// 1 State account instance == 1 Safe Pay instance
#[account]
pub struct Pool {
    
    // pool wallet token b
    pool_wallet_a: Pubkey,

    // pool wallet token a
    pool_wallet_b: Pubkey,
    
    // a * b = k
    k_constant: u64
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

#[account]
pub struct Deposit {
    amount: u16,
    depositor: Pubkey,
    mint_of_token_deposited: Pubkey,
    timestamp: i64,
}