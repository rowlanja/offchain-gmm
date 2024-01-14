use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock};
use anchor_spl::{token::{Mint, Token, TokenAccount, Transfer}};
use solana_program::{
    account_info::{AccountInfo},
    pubkey::{Pubkey, PUBKEY_BYTES},
    msg,
    program_memory::sol_memcmp,
    program_pack::{Pack},
};
pub use borsh::{BorshDeserialize, BorshSchema, BorshSerialize}; 


declare_id!("DJmR54jYwYvzAfFKCFrdpg5njsMyeAPyAEqt8usLkUE7");

#[program]
pub mod onchain_gmm_contracts {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreateLiquidityPool>,
        token_a_amount: u64,
        token_b_amount: u64,
        pubkey_invoker: Pubkey
    ) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_0.amount;
        let pool_balance = ctx.accounts.pool_wallet_token_0.amount;

        msg!("depositors balance [{}]", depositor_balance);
        msg!("pools balance [{}]", pool_balance);

        let _t0_mint = ctx.accounts.token0_mint.key().clone();
        let binding = ctx.accounts.user_wallet_token_0.key();
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
            from: ctx.accounts.user_wallet_token_0.to_account_info(),
            to: ctx.accounts.pool_wallet_token_0.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );

        anchor_spl::token::transfer(cpi_ctx, token_a_amount)?;

        // TRANSFER TOKEN B
        let _t1_mint = ctx.accounts.token1_mint.key().clone();
        let binding = ctx.accounts.user_wallet_token_1.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let outer = vec![inner.as_slice()];

        let transfer_instruction = Transfer{
            from: ctx.accounts.user_wallet_token_1.to_account_info(),
            to: ctx.accounts.pool_wallet_token_1.to_account_info(),
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
        pool.token0 = ctx.accounts.user_wallet_token_0.key();
        pool.token1 = ctx.accounts.user_wallet_token_1.key();
        pool.k_constant = token_a_amount * token_b_amount;
        pool.current_total_emissions = 0.0;
        
        // Time to save the deposit in PDA 
        let position = &mut ctx.accounts.position;
        position.amount = 100;
        position.timestamp = clock::Clock::get()
            .unwrap()
            .unix_timestamp
            .try_into()
            .unwrap();
        position.current_total_emissions =  pool.current_total_emissions;
        let timestamp = clock::Clock::get()
            .unwrap()
            .unix_timestamp
            .try_into()
            .unwrap();

        let stakers = &mut ctx.accounts.stakers_list.validators;
        stakers.push( ValidatorStakeInfo {
            token_0_amount: token_a_amount,
            token_1_amount: token_b_amount,
            timestamp,
            owner: pubkey_invoker
        });
        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        token_amount: u64,
        a_to_b: bool
    ) -> Result<()> {
        // print balances
        let pool_balance = ctx.accounts.pool_wallet_token_0.amount;
        msg!("[SWAP]pools balance [{}]", pool_balance);

        let pool_balance = ctx.accounts.pool_wallet_token_1.amount;
        msg!("[SWAP]pools balance [{}]", pool_balance);

        let _token0 = ctx.accounts.token0_mint.key().clone();
        let binding = ctx.accounts.user_wallet_token_0.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let _outer = vec![inner.as_slice()];

        // CALCULATE PRICE
        let k_constant = ctx.accounts.pool.k_constant;
        let token_a_pool_size = ctx.accounts.pool_wallet_token_0.amount;
        let token_b_pool_size = ctx.accounts.pool_wallet_token_1.amount;

        // WE NEED LOGIC TO DETERMIN SWAP FOR TOKEN(a) or TOKEN(b) [for now hardcode b] 
        if !a_to_b {
            let new_token_a_pool_size = token_a_pool_size + token_amount; 
            let new_token_b_pool_size = k_constant / new_token_a_pool_size; 
            let price = token_b_pool_size - new_token_b_pool_size;
    
            msg!("[SWAP] [TOKEN A SWAP] k constant [{}] price [{}]", k_constant, price);
        } else {
            let new_token_b_pool_size = token_b_pool_size + token_amount; 
            let new_token_a_pool_size = k_constant / new_token_b_pool_size; 
            let price = token_a_pool_size - new_token_a_pool_size;
    
            msg!("[SWAP] [TOKEN B SWAP] k constant [{}] price [{}]", k_constant, price);
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
}


#[account]
pub struct Pool {
    token0: Pubkey,
    token1: Pubkey,
    k_constant: u64,
    current_total_emissions: f64
}

#[account]
pub struct Position {
    amount: u16,
    timestamp: i64,
    current_total_emissions: f64
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8 + 8,
        seeds = [b"pool-state", user.key().as_ref()],
        bump
    )]
    pub pool_state: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        seeds=[b"pool_wallet_token_0".as_ref(), token0_mint.key().as_ref()],
        bump,
        token::mint=token0_mint,
        token::authority=user,
    )]
    pub pool_wallet_token_0: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds=[b"pool_wallet_token_1".as_ref(), token1_mint.key().as_ref()],
        bump,
        token::mint=token1_mint,
        token::authority=user,
    )]
    pub pool_wallet_token_1: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds=[b"position".as_ref(), user.key().as_ref(), token0_mint.key().as_ref()],
        space = 8 + 2 + 8 + 8,
        bump,
    )]
    pub position: Account<'info, Position>,

    #[account(
        init,
        payer = user,
        seeds=[b"validators".as_ref(), pool_state.key().as_ref()],
        space = 8000,
        bump,
    )]
    pub stakers_list:  Account<'info, ValidatorList>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_0: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_1: Account<'info, TokenAccount>,

    pub token0_mint: Account<'info, Mint>,   // USDC
    pub token1_mint: Account<'info, Mint>,   // ETH

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
        seeds = [b"pool-state", user.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        seeds=[b"pool_wallet_token_0".as_ref(), token0_mint.key().as_ref()],
        bump,
    )]
    pub pool_wallet_token_0: Account<'info, TokenAccount>,

    #[account(
        seeds=[b"pool_wallet_token_1".as_ref(), token1_mint.key().as_ref()],
        bump,
    )]
    pub pool_wallet_token_1: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_0: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_1: Account<'info, TokenAccount>,

    pub token0_mint: Account<'info, Mint>,   // USDC
    pub token1_mint: Account<'info, Mint>,   // ETH

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}


/// Storage list for all validator stake accounts in the pool.
#[derive(Default)]
#[account]
pub struct ValidatorList {
    /// List of stake info for each validator in the pool
    pub validators: Vec<ValidatorStakeInfo>,
}


#[derive(Default)]
#[account]
pub struct ValidatorStakeInfo {
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub owner: Pubkey,
    pub timestamp: u64
}

impl ValidatorList {
    /// Create an empty instance containing space for `max_validators` and
    /// preferred validator keys
    pub fn new(max_validators: u32) -> Self {
        Self {
            validators: vec![ValidatorStakeInfo::default(); max_validators as usize],
        }
    }

    /// Calculate the number of validator entries that fit in the provided
    /// length
    pub fn calculate_max_validators(buffer_length: usize) -> usize {
        buffer_length
            .saturating_div(24)
    }
}
