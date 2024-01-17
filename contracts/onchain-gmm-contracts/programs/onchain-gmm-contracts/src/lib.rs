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
        // let depositor_balance = ctx.accounts.user_wallet_token_0.amount;
        // let pool_balance = ctx.accounts.pool_wallet_token_0.amount;

        // msg!("depositors balance [{}]", depositor_balance);
        // msg!("pools balance [{}]", pool_balance);

        // let _t0_mint = ctx.accounts.token0_mint.key().clone();
        // let binding = ctx.accounts.user_wallet_token_0.key();
        // let inner = vec![
        //     b"state".as_ref(),
        //     binding.as_ref(),
        // ];
        // let outer = vec![inner.as_slice()];

        // TRANSFER TOKEN A

        // check provider has enough of token account a
        // move lp token account a to pool token account a
        // Below is the actual instruction that we are going to send to the Token program.
        // let transfer_instruction = Transfer{
        //     from: ctx.accounts.user_wallet_token_0.to_account_info(),
        //     to: ctx.accounts.pool_wallet_token_0.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );

        // anchor_spl::token::transfer(cpi_ctx, token_a_amount)?;

        // TRANSFER TOKEN B
        // let _t1_mint = ctx.accounts.token1_mint.key().clone();
        // let binding = ctx.accounts.user_wallet_token_1.key();
        // let inner = vec![
        //     b"state".as_ref(),
        //     binding.as_ref(),
        // ];
        // let outer = vec![inner.as_slice()];

        // let transfer_instruction = Transfer{
        //     from: ctx.accounts.user_wallet_token_1.to_account_info(),
        //     to: ctx.accounts.pool_wallet_token_1.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );

        // anchor_spl::token::transfer(cpi_ctx, token_b_amount)?;

        // Time to create the pool in PDA 
        // let pool = &mut ctx.accounts.pool_state;
        // pool.token0 = ctx.accounts.user_wallet_token_0.key();
        // pool.token1 = ctx.accounts.user_wallet_token_1.key();
        // pool.k_constant = token_a_amount * token_b_amount;
        // pool.current_total_emissions = 0.0;
        // pool.total_staked_token0 += token_a_amount as f64;
        // pool.total_staked_token1 += token_b_amount as f64;
        
        // // Time to save the deposit in PDA 
        // let position = &mut ctx.accounts.position;
        // position.amount = 100;
        // position.timestamp = clock::Clock::get()
        //     .unwrap()
        //     .unix_timestamp
        //     .try_into()
        //     .unwrap();
        // position.current_total_emissions =  pool.current_total_emissions;
        // let timestamp = clock::Clock::get()
        //     .unwrap()
        //     .unix_timestamp
        //     .try_into()
        //     .unwrap();

        // let stakers = &mut ctx.accounts.stakers_list.validators;
        // stakers.push( ValidatorStakeInfo {
        //     token_0_amount: token_a_amount as i64,
        //     token_1_amount: token_b_amount as i64,
        //     token_0_reward: 0.0,
        //     token_1_reward: 0.0,
        //     timestamp,
        //     owner: pubkey_invoker
        // });
        Ok(())
    }

    pub fn deposit(
        ctx: Context<CreateLiquidityPool>,
        token_a_amount: u64,
        token_b_amount: u64,
        pubkey_invoker: Pubkey
    ) -> Result<()> {
        // print balances
        // let depositor_balance = ctx.accounts.user_wallet_token_0.amount;
        // let pool_balance = ctx.accounts.pool_wallet_token_0.amount;

        // msg!("depositors balance [{}]", depositor_balance);
        // msg!("pools balance [{}]", pool_balance);

        // let _t0_mint = ctx.accounts.token0_mint.key().clone();
        // let binding = ctx.accounts.user_wallet_token_0.key();
        // let inner = vec![
        //     b"state".as_ref(),
        //     binding.as_ref(),
        // ];
        // let outer = vec![inner.as_slice()];

        // TRANSFER TOKEN A

        // check provider has enough of token account a
        // move lp token account a to pool token account a
        // Below is the actual instruction that we are going to send to the Token program.
        // let transfer_instruction = Transfer{
        //     from: ctx.accounts.user_wallet_token_0.to_account_info(),
        //     to: ctx.accounts.pool_wallet_token_0.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );

        // anchor_spl::token::transfer(cpi_ctx, token_a_amount)?;

        // TRANSFER TOKEN B
        // let _t1_mint = ctx.accounts.token1_mint.key().clone();
        // let binding = ctx.accounts.user_wallet_token_1.key();
        // let inner = vec![
        //     b"state".as_ref(),
        //     binding.as_ref(),
        // ];
        // let outer = vec![inner.as_slice()];

        // let transfer_instruction = Transfer{
        //     from: ctx.accounts.user_wallet_token_1.to_account_info(),
        //     to: ctx.accounts.pool_wallet_token_1.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     transfer_instruction,
        //     outer.as_slice(),
        // );

        // anchor_spl::token::transfer(cpi_ctx, token_b_amount)?;

        // Time to create the pool in PDA 
        // let pool = &mut ctx.accounts.pool_state;
        // pool.token0 = ctx.accounts.user_wallet_token_0.key();
        // pool.token1 = ctx.accounts.user_wallet_token_1.key();
        // pool.k_constant = token_a_amount * token_b_amount;
        // pool.current_total_emissions = 0.0;
        // pool.total_staked_token0 += token_a_amount as f64;
        // pool.total_staked_token1 += token_b_amount as f64;
        
        // // Time to save the deposit in PDA 
        // let position = &mut ctx.accounts.position;
        // position.amount = 100;
        // position.timestamp = clock::Clock::get()
        //     .unwrap()
        //     .unix_timestamp
        //     .try_into()
        //     .unwrap();
        // position.current_total_emissions =  pool.current_total_emissions;
        // let timestamp = clock::Clock::get()
        //     .unwrap()
        //     .unix_timestamp
        //     .try_into()
        //     .unwrap();

        // let stakers = &mut ctx.accounts.stakers_list.validators;
        // stakers.push( ValidatorStakeInfo {
        //     token_0_amount: token_a_amount as i64,
        //     token_1_amount: token_b_amount as i64,
        //     token_0_reward: 0.0,
        //     token_1_reward: 0.0,
        //     timestamp,
        //     owner: pubkey_invoker
        // });
        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
        input_amount: u64,
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
        let outer = vec![inner.as_slice()];

        // CALCULATE PRICE
        let k_constant = ctx.accounts.pool.k_constant;
        let token_a_pool_size = ctx.accounts.pool_wallet_token_0.amount;
        let token_b_pool_size = ctx.accounts.pool_wallet_token_1.amount;
        // WE NEED LOGIC TO DETERMIN SWAP FOR TOKEN(a) or TOKEN(b) [for now hardcode b] 
        let new_token_a_pool_size: u64;
        let new_token_b_pool_size: u64;
        let output_amount = if !a_to_b {
            new_token_a_pool_size = token_a_pool_size + input_amount; 
            new_token_b_pool_size = k_constant / new_token_a_pool_size; 
            token_b_pool_size - new_token_b_pool_size
        } else {
            new_token_b_pool_size = token_b_pool_size + input_amount; 
            new_token_a_pool_size = k_constant / new_token_b_pool_size; 
            token_a_pool_size - new_token_a_pool_size
        };
        msg!("[SWAP] [TOKEN A SWAP] k constant [{}] price [{}]", k_constant, output_amount);
        const fee: f64 = 0.05;
        const perc_to_swapper: f64 = 1.0 - fee;
        let real_output: f64 = perc_to_swapper * output_amount as f64;
        let fee_output: f64 = output_amount as f64 - real_output;
        msg!("[SWAP] real output [{}] fee_output [{}] ", real_output, fee_output);

        // TRANSFER TOKEN A to POOL

        // check provider has enough of token account a
        // move lp token account a to pool token account a
        // Below is the actual instruction that we are going to send to the Token program.
        let binding = ctx.accounts.user_wallet_token_0.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let outer = vec![inner.as_slice()];
        
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

        anchor_spl::token::transfer(cpi_ctx, input_amount)?;

        // TRANSFER TOKEN B to USER

        // check provider has enough of token account b
        // move lp token account a to pool token account b
        // Below is the actual instruction that we are going to send to the Token program.
        let binding = ctx.accounts.user_wallet_token_1.key();
        let inner = vec![
            b"state".as_ref(),
            binding.as_ref(),
        ];
        let outer = vec![inner.as_slice()];
        
        let transfer_instruction = Transfer{
            from: ctx.accounts.pool_wallet_token_1.to_account_info(),
            to: ctx.accounts.user_wallet_token_1.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );
        // APOLOGIES TO GOD FOR THIS CODE
        anchor_spl::token::transfer(cpi_ctx, real_output as u64)?;
        let stakers = &mut ctx.accounts.stakers_list.validators;
        let staker_len = stakers.len();
        let mut iter = &mut stakers.iter_mut();
        // Time to update the stakers rewards 
        while let Some(staker) = iter.next() {
            msg!("staker vec[{}] [{}] [{}]", staker.owner, staker.token_0_amount, staker.token_1_amount);
            if a_to_b {
                staker.token_0_reward = (staker.token_0_amount / new_token_a_pool_size as i64 ) as f64 * fee_output;
            } else { 
                staker.token_1_reward = (staker.token_1_amount / new_token_b_pool_size as i64 ) as f64 * fee_output;
            }
        }
        Ok(())
    }
}


#[account]
pub struct Pool {
    is_init: bool,
    token0: Pubkey,
    token1: Pubkey,
    k_constant: u64,
    current_total_emissions: f64,
    total_staked_token0: f64,
    total_staked_token1: f64
}

#[account]
pub struct Position {
    amount: u16,
    timestamp: i64,
    current_total_emissions: f64
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub pool_state: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        seeds=[b"position".as_ref(), user.key().as_ref(), pool_state.key().as_ref()],
        space = 8 + 2 + 8 + 8,
        bump,
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub stakers_list:  Account<'info, ValidatorList>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_0: Account<'info, TokenAccount>,

    // Alice's USDC wallet that has already approved the escrow wallet
    #[account(mut)]
    pub user_wallet_token_1: Account<'info, TokenAccount>,

    // Application level accounts
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    // Users and accounts in the system
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 1 + 32 + 32 + 8 + 8 + 8 + 8,
        seeds = [b"pool-state", user.key().as_ref()],
        bump
    )]
    pub pool_state: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        seeds=[b"stakers".as_ref(), pool_state.key().as_ref()],
        space = 80000,
        bump,
    )]
    pub stakers_list:  Account<'info, ValidatorList>,

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

    #[account(
        mut,
        seeds=[b"stakers".as_ref(), pool.key().as_ref()],
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


/// Storage list for all validator stake accounts in the pool.
#[derive(Default)]
#[account]
pub struct ValidatorList {
    /// List of stake info for each validator in the pool
    pub validators: Vec<ValidatorStakeInfo>,
}


#[derive(Default, BorshSerialize, BorshDeserialize)]
#[zero_copy]
pub struct ValidatorStakeInfo {
    pub token_0_amount: i64,
    pub token_1_amount: i64,
    pub token_0_reward: f64,
    pub token_1_reward: f64,
    pub owner: Pubkey,
    pub timestamp: i64
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
