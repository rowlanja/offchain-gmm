use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock, log::sol_log};
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};

declare_id!("DJmR54jYwYvzAfFKCFrdpg5njsMyeAPyAEqt8usLkUE7");

#[program]
pub mod onchain_gmm_contracts {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreateLiquidityPool>,
        lower_bound: f64,
        upper_bound: f64,
        fee_percent: f64,
        token_a_amount: u64,
        token_b_amount: u64,
    ) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_a.amount;
        let pool_balance = ctx.accounts.pool_wallet_token_a.amount;

        msg!("depositors balance [{}]", depositor_balance);
        msg!("pools balance [{}]", pool_balance);

        let mint_of_token_being_sent_pk_a = ctx.accounts.mint_of_token_being_sent_a.key().clone();
        let binding = ctx.accounts.user_wallet_token_a.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        // TRANSFER TOKEN A

        // check provider has enough of token account a
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

        anchor_spl::token::transfer(cpi_ctx, token_a_amount)?;

        // TRANSFER TOKEN B

        // check provider has enough of token account b
        // move lp token account a to pool token account b
        // Below is the actual instruction that we are going to send to the Token program.
        let transfer_instruction = Transfer{
            from: ctx.accounts.user_wallet_token_b.to_account_info(),
            to: ctx.accounts.pool_wallet_token_b.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        anchor_spl::token::transfer(cpi_ctx, token_b_amount)?;

        // Time to create the pool in PDA 
        let pool = &mut ctx.accounts.pool_state;
        pool.pool_wallet_a = ctx.accounts.user_wallet_token_a.key();
        pool.pool_wallet_b = ctx.accounts.user_wallet_token_b.key();
        pool.k_constant = token_a_amount * token_b_amount;
        
        // Time to save the deposit in PDA 
        let deposit = &mut ctx.accounts.stake_record;
        deposit.amount = 100;
        deposit.timestamp = clock::Clock::get()
            .unwrap()
            .unix_timestamp
            .try_into()
            .unwrap();

        deposit.lower_bound = f64::from(lower_bound);
        deposit.upper_bound = f64::from(upper_bound);
        deposit.fee_percent = f64::from(fee_percent);

        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        token_amount: u64,
        token_pubkey: Pubkey
    ) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_a.amount;
        let pool_balance = ctx.accounts.pool_wallet_token_a.amount;

        msg!("depositors balance [{}]", depositor_balance);
        msg!("pools balance [{}]", pool_balance);
        let mint_of_token_being_sent_pk_a = ctx.accounts.mint_of_token_being_sent_a.key().clone();
        let binding = ctx.accounts.user_wallet_token_a.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        // CALCULATE PRICE
        let k_constant = ctx.accounts.pool.k_constant;
        let token_a_pool_size = ctx.accounts.pool_wallet_token_a.amount;
        let token_b_pool_size = ctx.accounts.pool_wallet_token_b.amount;

        // WE NEED LOGIC TO DETERMIN SWAP FOR TOKEN(a) or TOKEN(b) [for now hardcode b] 
        if token_pubkey == ctx.accounts.mint_of_token_being_sent_a.key().clone() {
            let new_token_a_pool_size = token_a_pool_size + token_amount; 
            let new_token_b_pool_size = k_constant / new_token_a_pool_size; 
            let price = token_b_pool_size - new_token_b_pool_size;
    
            msg!("[TOKEN A PRICE] : k constant [{}] price [{}]", k_constant, price);
        } else if token_pubkey == ctx.accounts.mint_of_token_being_sent_b.key().clone() {
            let new_token_b_pool_size = token_b_pool_size + token_amount; 
            let new_token_a_pool_size = k_constant / new_token_b_pool_size; 
            let price = token_a_pool_size - new_token_a_pool_size;
    
            msg!("[TOKEN B PRICE] : k constant [{}] price [{}]", k_constant, price);
        } else {
            msg!("incorrect token pubkey");

        }

        // TRANSFER TOKEN A to POOL

        // check provider has enough of token account a
        // move lp token account a to pool token account a
        // Below is the actual instruction that we are going to send to the Token program.
        // let transfer_instruction = Transfer{
        //     from: ctx.accounts.user_wallet_token_a.to_account_info(),
        //     to: ctx.accounts.pool_wallet_token_a.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );

        // anchor_spl::token::transfer(cpi_ctx, token_a_amount)?;

        // TRANSFER TOKEN B to USER

        // check provider has enough of token account b
        // move lp token account a to pool token account b
        // Below is the actual instruction that we are going to send to the Token program.
        // let transfer_instruction = Transfer{
        //     from: ctx.accounts.user_wallet_token_b.to_account_info(),
        //     to: ctx.accounts.pool_wallet_token_b.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );

        // anchor_spl::token::transfer(cpi_ctx, token_b_amount)?;

        // // Time to create the pool in PDA 
        // let pool = &mut ctx.accounts.pool_state;
        // pool.pool_wallet_a = ctx.accounts.user_wallet_token_a;
        // pool.pool_wallet_b = ctx.accounts.user_wallet_token_b;
        // pool.k_constant = token_a_amount * token_b_amount;
        
        // // Time to save the deposit in PDA 
        // let deposit = &mut ctx.accounts.stake_record;
        // deposit.amount = 100;
        // deposit.timestamp = clock::Clock::get()
        //     .unwrap()
        //     .unix_timestamp
        //     .try_into()
        //     .unwrap();

        // deposit.lower_bound = f64::from(lower_bound);
        // deposit.upper_bound = f64::from(upper_bound);
        // deposit.fee_percent = f64::from(fee_percent);

        Ok(())
    }

    // pub fn deposit_liquidity(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
    //     // print balances
    //     let depositor_balance = ctx.accounts.user_wallet_token_a.amount;
    //     // let pool_balance = ctx.accounts.pool_wallet_token_b.amount;

    //     msg!("depositor balance [{}]", depositor_balance);

    //     let pool = ctx.accounts.pool.clone();
    //     let pool_token_a_balance = ctx.accounts.pool_wallet_token_a.amount;
    //     let pool_token_b_balance = ctx.accounts.pool_wallet_token_b.amount;
    //     println!("pool balance [{}]", pool_token_a_balance);
    //     println!("pool balance [{}]", pool_token_b_balance);
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

//         msg!("deposit balance [{}]", depositor_balance);
//         // println!("pool balance [{}]", pool_balance);
//         Ok(())
//     }
}

// #[derive(Accounts)]
// pub struct DepositLiquidity<'info> {
//     //  Derived PDAs
//     #[account(
//         mut,
//         seeds=[b"state".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
//         bump,
//     )]
//     pool: Account<'info, Pool>,

//     #[account(
//         mut,
//         seeds=[b"pool_wallet_token_a".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
//         bump
//     )]
//     pool_wallet_token_a: Account<'info, TokenAccount>,

//     #[account(
//         mut,
//         seeds=[b"pool_wallet_token_b".as_ref(), mint_of_token_being_sent_b.key().as_ref()],
//         bump
//     )]
//     pool_wallet_token_b: Account<'info, TokenAccount>,
    
//     #[account(mut)]
//     depositor: Signer<'info>,                     // Alice
   
//     // Alice's USDC wallet that has already approved the escrow wallet
//     #[account(
//         mut,
//         constraint=user_wallet_token_a.owner == depositor.key(),
//         constraint=user_wallet_token_a.mint == mint_of_token_being_sent_a.key()
//     )]
//     user_wallet_token_a: Account<'info, TokenAccount>,

//     // Alice's USDC wallet that has already approved the escrow wallet
//     #[account(
//             mut,
//             constraint=user_wallet_token_b.owner == depositor.key(),
//             constraint=user_wallet_token_b.mint == mint_of_token_being_sent_b.key()
//     )]
//     user_wallet_token_b: Account<'info, TokenAccount>,

//     mint_of_token_being_sent_a: Account<'info, Mint>,   // USDC
//     mint_of_token_being_sent_b: Account<'info, Mint>,   // ETH


//     // // Application level accounts
//     system_program: Program<'info, System>,
//     token_program: Program<'info, Token>,
//     // rent: Sysvar<'info, Rent>,
// }

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1,
        seeds = [b"user-stats", user.key().as_ref()],
        bump
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

    #[account(
        init,
        payer = user,
        seeds=[b"pool_wallet_token_b".as_ref(), mint_of_token_being_sent_b.key().as_ref()],
        bump,
        token::mint=mint_of_token_being_sent_b,
        token::authority=pool_state,
    )]
    pub pool_wallet_token_b: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds=[b"stake".as_ref(), user.key().as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        space = 8 + 2 + 8 + 8 + 8 + 8,
        bump,
    )]
    pub stake_record: Account<'info, Deposit>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_a: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_b: Account<'info, TokenAccount>,

    // // Alice's USDC wallet that has already approved the escrow wallet
    // #[account(
    //         mut,
    //         constraint=user_wallet_token_b.owner == owner.key(),
    //         constraint=user_wallet_token_b.mint == mint_of_token_being_sent_b.key()
    // )]
    // user_wallet_token_b: Account<'info, TokenAccount>,

    pub mint_of_token_being_sent_a: Account<'info, Mint>,   // USDC
    pub mint_of_token_being_sent_b: Account<'info, Mint>,   // ETH

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user-stats", user.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        seeds=[b"pool_wallet_token_a".as_ref(), mint_of_token_being_sent_a.key().as_ref()],
        bump,
    )]
    pub pool_wallet_token_a: Account<'info, TokenAccount>,

    #[account(
        seeds=[b"pool_wallet_token_b".as_ref(), mint_of_token_being_sent_b.key().as_ref()],
        bump,
    )]
    pub pool_wallet_token_b: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_a: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_b: Account<'info, TokenAccount>,

    // // Alice's USDC wallet that has already approved the escrow wallet
    // #[account(
    //         mut,
    //         constraint=user_wallet_token_b.owner == owner.key(),
    //         constraint=user_wallet_token_b.mint == mint_of_token_being_sent_b.key()
    // )]
    // user_wallet_token_b: Account<'info, TokenAccount>,

    pub mint_of_token_being_sent_a: Account<'info, Mint>,   // USDC
    pub mint_of_token_being_sent_b: Account<'info, Mint>,   // ETH

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    
    // pool wallet token b
    pool_wallet_a: Pubkey,

    // pool wallet token a
    pool_wallet_b: Pubkey,
    
    // a * b = k
    k_constant: u64
}

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
    timestamp: i64,
    lower_bound: f64,
    upper_bound: f64,
    fee_percent: f64
}

pub fn create_concentrated_pool(
    ctx: Context<CreateConcentrateLiquidityPool>,
) -> Result<()> {

    Ok(())
}

pub fn swap_concentrated_pool(
    ctx: Context<SwapConcentratedLiquidityPool>,
) -> Result<()> {

    Ok(())
}

#[derive(Accounts)]
pub struct CreateConcentrateLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SwapConcentratedLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}
