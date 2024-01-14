use {
    crate::big_vec::BigVec,
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    bytemuck::{Pod, Zeroable},
    num_derive::{FromPrimitive, ToPrimitive},
    num_traits::{FromPrimitive, ToPrimitive},
    solana_program::{
        account_info::AccountInfo,
        borsh0_10::get_instance_packed_len,
        msg,
        program_error::ProgramError,
        program_memory::sol_memcmp,
        program_pack::{Pack, Sealed},
        pubkey::{Pubkey, PUBKEY_BYTES},
        stake::state::Lockup,
    },
    spl_pod::primitives::{PodU32, PodU64},
    spl_token_2022::{
        extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
        state::{Account, AccountState, Mint},
    },
    std::{borrow::Borrow, convert::TryFrom, fmt, matches},
};
use anchor_lang::prelude::*;
/// Initialized program details.
#[repr(C)]
#[derive(Accounts)]
pub struct StakePool {
    /// Validator stake list storage account
    pub validator_list: Pubkey,

    /// Total stake under management.
    pub last_update_epoch: u64
}

/// Checks if the given extension is supported for the stake pool mint
pub fn is_extension_supported_for_mint(extension_type: &ExtensionType) -> bool {
    const SUPPORTED_EXTENSIONS: [ExtensionType; 8] = [
        ExtensionType::Uninitialized,
        ExtensionType::TransferFeeConfig,
        ExtensionType::ConfidentialTransferMint,
        ExtensionType::ConfidentialTransferFeeConfig,
        ExtensionType::DefaultAccountState, // ok, but a freeze authority is not
        ExtensionType::InterestBearingConfig,
        ExtensionType::MetadataPointer,
        ExtensionType::TokenMetadata,
    ];
    if !SUPPORTED_EXTENSIONS.contains(extension_type) {
        msg!(
            "Stake pool mint account cannot have the {:?} extension",
            extension_type
        );
        false
    } else {
        true
    }
}

/// Checks if the given extension is supported for the stake pool's fee account
pub fn is_extension_supported_for_fee_account(extension_type: &ExtensionType) -> bool {
    // Note: this does not include the `ConfidentialTransferAccount` extension
    // because it is possible to block non-confidential transfers with the
    // extension enabled.
    const SUPPORTED_EXTENSIONS: [ExtensionType; 4] = [
        ExtensionType::Uninitialized,
        ExtensionType::TransferFeeAmount,
        ExtensionType::ImmutableOwner,
        ExtensionType::CpiGuard,
    ];
    if !SUPPORTED_EXTENSIONS.contains(extension_type) {
        msg!("Fee account cannot have the {:?} extension", extension_type);
        false
    } else {
        true
    }
}

/// Storage list for all validator stake accounts in the pool.
#[repr(C)]
#[derive(Accounts)]
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
#[repr(C)]
#[derive(Accounts)]
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
            .saturating_div(ValidatorStakeInfo::LEN)
    }

    /// Check if contains validator with particular pubkey
    pub fn contains(&self, vote_account_address: &Pubkey) -> bool {
        self.validators
            .iter()
            .any(|x| x.vote_account_address == *vote_account_address)
    }

    /// Check if contains validator with particular pubkey
    pub fn find_mut(&mut self, vote_account_address: &Pubkey) -> Option<&mut ValidatorStakeInfo> {
        self.validators
            .iter_mut()
            .find(|x| x.vote_account_address == *vote_account_address)
    }
    /// Check if contains validator with particular pubkey
    pub fn find(&self, vote_account_address: &Pubkey) -> Option<&ValidatorStakeInfo> {
        self.validators
            .iter()
            .find(|x| x.vote_account_address == *vote_account_address)
    }

    /// Check if the list has any active stake
    pub fn has_active_stake(&self) -> bool {
        self.validators
            .iter()
            .any(|x| u64::from(x.active_stake_lamports) > 0)
    }
}
