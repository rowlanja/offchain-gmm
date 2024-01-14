use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock, log::sol_log};
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::{try_from_slice_unchecked, get_instance_packed_len},
    pubkey::{Pubkey, PUBKEY_BYTES},
    msg,
    program_error::ProgramError,
    program_memory::sol_memcmp,
    program_pack::{Pack, Sealed},
};
pub use borsh::{BorshDeserialize, BorshSchema, BorshSerialize}; 
pub mod big_vec;

declare_id!("DJmR54jYwYvzAfFKCFrdpg5njsMyeAPyAEqt8usLkUE7");

#[program]
pub mod onchain_gmm_contracts {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreateLiquidityPool>,
        token_a_amount: u64,
        token_b_amount: u64,
    ) -> Result<()> {
        // print balances
        let depositor_balance = ctx.accounts.user_wallet_token_0.amount;
        let pool_balance = ctx.accounts.pool_wallet_token_0.amount;

        msg!("depositors balance [{}]", depositor_balance);
        msg!("pools balance [{}]", pool_balance);

        let t0_mint = ctx.accounts.token0_mint.key().clone();
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
        let t1_mint = ctx.accounts.token1_mint.key().clone();
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

        let token0 = ctx.accounts.token0_mint.key().clone();
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
    current_total_emissions: f64,
    stakers_list: Pubkey
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
        space = 8 + 32 + 32 + 8 + 8 + 32,
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
        space = 8 + 2 + 8 + 8,
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

/// Information about a validator in the pool
///
/// NOTE: ORDER IS VERY IMPORTANT HERE, PLEASE DO NOT RE-ORDER THE FIELDS UNLESS
/// THERE'S AN EXTREMELY GOOD REASON.
///
/// To save on BPF instructions, the serialized bytes are reinterpreted with a
/// bytemuck transmute, which means that this structure cannot have any
/// undeclared alignment-padding in its representation.
#[derive(Default)]
#[account]
pub struct ValidatorStakeInfo {
    /// Amount of lamports on the validator stake account, including rent
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub active_stake_lamports: u64,

    /// Amount of transient stake delegated to this validator
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub transient_stake_lamports: u64,

    /// Last epoch the active and transient stake lamports fields were updated
    pub last_update_epoch: u64
}

impl ValidatorStakeInfo {
    /// Get the total lamports on this validator (active and transient)
    pub fn stake_lamports(&self) -> u64 {
        u64::from(self.active_stake_lamports)
            .checked_add(self.transient_stake_lamports.into())
            .unwrap()
    }

    /// Performs a very cheap comparison, for checking if this validator stake
    /// info matches the vote account address
    pub fn memcmp_pubkey(data: &[u8], vote_address: &Pubkey) -> bool {
        sol_memcmp(
            &data[41..41_usize.saturating_add(PUBKEY_BYTES)],
            vote_address.as_ref(),
            PUBKEY_BYTES,
        ) == 0
    }

    /// Performs a comparison, used to check if this validator stake
    /// info has more active lamports than some limit
    pub fn active_lamports_greater_than(data: &[u8], lamports: &u64) -> bool {
        // without this unwrap, compute usage goes up significantly
        u64::try_from_slice(&data[0..8]).unwrap() > *lamports
    }

    /// Performs a comparison, used to check if this validator stake
    /// info has more transient lamports than some limit
    pub fn transient_lamports_greater_than(data: &[u8], lamports: &u64) -> bool {
        // without this unwrap, compute usage goes up significantly
        u64::try_from_slice(&data[8..16]).unwrap() > *lamports
    }
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

    // Check if contains validator with particular pubkey
    // pub fn contains(&self, vote_account_address: &Pubkey) -> bool {
    //     self.validators
    //         .iter()
    //         .any(|x| x.vote_account_address == *vote_account_address)
    // }

    // Check if contains validator with particular pubkey
    // pub fn find_mut(&mut self, vote_account_address: &Pubkey) -> Option<&mut ValidatorStakeInfo> {
    //     self.validators
    //         .iter_mut()
    //         .find(|x| x.vote_account_address == *vote_account_address)
    // }
    // Check if contains validator with particular pubkey
    // pub fn find(&self, vote_account_address: &Pubkey) -> Option<&ValidatorStakeInfo> {
    //     self.validators
    //         .iter()
    //         .find(|x| x.vote_account_address == *vote_account_address)
    // }

    // Check if the list has any active stake
    // pub fn has_active_stake(&self) -> bool {
    //     self.validators
    //         .iter()
    //         .any(|x| u64::from(x.active_stake_lamports) > 0)
    // }
}
