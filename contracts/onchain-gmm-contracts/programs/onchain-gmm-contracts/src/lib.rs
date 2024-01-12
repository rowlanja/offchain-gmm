use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock};
use anchor_spl::{token::{Mint, Transfer}};
use anchor_spl::token::{Token, TokenAccount};


pub mod errors;
pub mod instructions;
pub mod math;
pub mod state;
pub mod util;
pub mod manager;
pub mod constants;

use instructions::*;
use crate::state::{OpenPositionBumps, WhirlpoolBumps};

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

        let _mint_of_token_being_sent_pk_a = ctx.accounts.mint_of_token_being_sent_a.key().clone();
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
        ctx: Context<V1Swap>,
        token_amount: u64,
        token_pubkey: Pubkey
    ) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_a.amount;
        let pool_balance = ctx.accounts.pool_wallet_token_a.amount;

        msg!("depositors balance [{}]", depositor_balance);
        msg!("pools balance [{}]", pool_balance);
        let _mint_of_token_being_sent_pk_a = ctx.accounts.mint_of_token_being_sent_a.key().clone();
        let binding = ctx.accounts.user_wallet_token_a.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let _outer = vec![inner.as_slice()];

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
// read https://uniswapv3book.com/milestone_1/calculating-liquidity.html


    // pub fn price_to_sqrtp() -> f64 {
    //     let q96: i64 = 2_i64.pow(96);

    //     return q96;
    // }

    // pub fn swap_concentrated_pool(
    //     ctx: Context<CreatePositionConcentrateLiquidityPool>,
    //     current_price: f64, // this should be loaded from pool
    //     upper_price_bound: f64,
    //     lower_price_bound: f64
    // ) -> Result<()> {
    //     // const Q96: U256 = U256([0, 4294967296, 0, 0]);
    //     let current_tick = price_to_tick(current_price);
    //     let upper_tick = price_to_tick(upper_price_bound);
    //     let lower_tick = price_to_tick(lower_price_bound);
    //     msg!("current tick : [{}] upper tick :  [{}] lower tick : [{}]", current_tick, upper_tick, lower_tick);
        
    //     // Last thing, we use Q64.96 to store sqrt(P)
        
    // //    let q96: i64 = 2i64.pow(96);
    //     let sqrtp_price: f64 = price_to_sqrtp(current_price);
    //     msg!("current sqrtp_price : [{}] ", sqrtp_price);
    //     Ok(())
    // }


    pub fn open_position(
        ctx: Context<OpenPosition>,
        bumps: OpenPositionBumps,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> Result<()> {
        return instructions::open_position::handler(
            ctx,
            bumps,
            tick_lower_index,
            tick_upper_index,
        );
    }

    pub fn increase_liquidity(
        ctx: Context<ModifyLiquidity>,
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
    ) -> Result<()> {
        return instructions::increase_liquidity::handler(
            ctx,
            liquidity_amount,
            token_max_a,
            token_max_b,
        );
    }

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        bumps: WhirlpoolBumps,
        tick_spacing: u16,
        initial_sqrt_price: u128,
    ) -> Result<()> {
        return instructions::initialize_pool::handler(
            ctx,
            bumps,
            tick_spacing,
            initial_sqrt_price,
        );
    }

    pub fn clammSwap(
        ctx: Context<Swap>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool, // Zero for one
    ) -> Result<()> {
        return instructions::swap::handler(
            ctx,
            amount,
            other_amount_threshold,
            sqrt_price_limit,
            amount_specified_is_input,
            a_to_b,
        );
    }
}

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
pub struct V1Swap<'info> {
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

#[account]
pub struct Tick {
    initialized: bool,
    liquidity: u128
}

#[account]
pub struct Position {
    liquidity: u128,
}

#[derive(Accounts)]
#[instruction(lower_tick_id: u64, upper_tick_id: u64)]
pub struct CreatePositionConcentrateLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds=[b"clamm".as_ref(), token0.key().as_ref(), token1.key().as_ref(), &lower_tick_id.to_le_bytes()],
        bump,
        space = 8 + 1 + 16
    )]
    pub lower_tick: Account<'info, Tick>,

    #[account(
        init,
        payer = user,
        seeds=[b"clamm".as_ref(), token0.key().as_ref(), token1.key().as_ref(), &upper_tick_id.to_le_bytes()],
        bump,
        space = 8 + 1 + 16
    )]
    pub upper_tick: Account<'info, Tick>,

    #[account(
        init,
        payer = user,
        seeds=[b"clamm_position".as_ref(), token0.key().as_ref(), token1.key().as_ref(), &lower_tick_id.to_le_bytes(), &upper_tick_id.to_le_bytes()],
        bump,
        space = 8 + 16
    )]
    pub position: Account<'info, Position>,

    #[account(
        init,
        payer = user,
        seeds=[b"pool_wallet_token_a".as_ref(), token0.key().as_ref()],
        bump,
        token::mint=token0,
        token::authority=user,
    )]
    pub pool_wallet_token_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds=[b"pool_wallet_token_a".as_ref(), token0.key().as_ref()],
        bump,
        token::mint=token0,
        token::authority=user,
    )]
    pub pool_wallet_token_b: Account<'info, TokenAccount>,

    pub token0: Account<'info, Mint>,   // USDC
    pub token1: Account<'info, Mint>,   // ETH
    
    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_a: Account<'info, TokenAccount>,

    // Alice's ETH wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_b: Account<'info, TokenAccount>,


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

#[derive(Accounts)]
pub struct CreateConcentrateLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    pub token0: Account<'info, Mint>,   // USDC
    pub token1: Account<'info, Mint>,   // ETH

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}



#[account]
#[derive(Default)] 
pub struct ConcentratedPool { 

    // A primary key that allows us to derive other important accounts
    idx: u64,
    
    // Alice
    user_sending: Pubkey,

    // Bob
    user_receiving: Pubkey,

    token0: Pubkey,
    token1: Pubkey,

    // The escrow wallet
    escrow_wallet: Pubkey,

    // The amount of tokens Alice wants to send to Bob
    amount_tokens: u64,

    // An enumm that is to represent some kind of state machine
    stage: u8,
}

#[cfg(test)]
mod position_bundle_initialize_tests {
    use super::*;


    #[test]
    fn test_default() {
        let position_bundle = PositionBundle {
            ..Default::default()
        };
        assert_eq!(position_bundle.position_bundle_mint, Pubkey::default());
        for bitmap in position_bundle.position_bitmap.iter() {
            assert_eq!(*bitmap, 0);
        }
    }
}