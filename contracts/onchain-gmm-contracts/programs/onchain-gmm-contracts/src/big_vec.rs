//! Big vector type, used with vectors that can't be serde'd
#![allow(clippy::arithmetic_side_effects)] // checked math involves too many compute units

use {
    arrayref::array_ref,
    borsh::BorshDeserialize,
    bytemuck::Pod,
    solana_program::{program_error::ProgramError, program_memory::sol_memmove},
    std::mem,
};

/// Contains easy to use utilities for a big vector of Borsh-compatible types,
/// to avoid managing the entire struct on-chain and blow through stack limits.
pub struct BigVec<'data> {
    /// Underlying data buffer, pieces of which are serialized
    pub data: &'data mut [u8],
}

const VEC_SIZE_BYTES: usize = 4;

impl<'data> BigVec<'data> {
    /// Get the length of the vector
    pub fn len(&self) -> u32 {
        let vec_len = array_ref![self.data, 0, VEC_SIZE_BYTES];
        u32::from_le_bytes(*vec_len)
    }

    /// Find out if the vector has no contents (as demanded by clippy)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retain all elements that match the provided function, discard all others
    pub fn retain<T: Pod, F: Fn(&[u8]) -> bool>(
        &mut self,
        predicate: F,
    ) -> Result<(), ProgramError> {
        let mut vec_len = self.len();
        let mut removals_found = 0;
        let mut dst_start_index = 0;

        let data_start_index = VEC_SIZE_BYTES;
        let data_end_index =
            data_start_index.saturating_add((vec_len as usize).saturating_mul(mem::size_of::<T>()));
        for start_index in (data_start_index..data_end_index).step_by(mem::size_of::<T>()) {
            let end_index = start_index + mem::size_of::<T>();
            let slice = &self.data[start_index..end_index];
            if !predicate(slice) {
                let gap = removals_found * mem::size_of::<T>();
                if removals_found > 0 {
                    // In case the compute budget is ever bumped up, allowing us
                    // to use this safe code instead:
                    // self.data.copy_within(dst_start_index + gap..start_index, dst_start_index);
                    unsafe {
                        sol_memmove(
                            self.data[dst_start_index..start_index - gap].as_mut_ptr(),
                            self.data[dst_start_index + gap..start_index].as_mut_ptr(),
                            start_index - gap - dst_start_index,
                        );
                    }
                }
                dst_start_index = start_index - gap;
                removals_found += 1;
                vec_len -= 1;
            }
        }

        // final memmove
        if removals_found > 0 {
            let gap = removals_found * mem::size_of::<T>();
            // In case the compute budget is ever bumped up, allowing us
            // to use this safe code instead:
            //    self.data.copy_within(
            //        dst_start_index + gap..data_end_index,
            //        dst_start_index,
            //    );
            unsafe {
                sol_memmove(
                    self.data[dst_start_index..data_end_index - gap].as_mut_ptr(),
                    self.data[dst_start_index + gap..data_end_index].as_mut_ptr(),
                    data_end_index - gap - dst_start_index,
                );
            }
        }

        let mut vec_len_ref = &mut self.data[0..VEC_SIZE_BYTES];
        borsh::to_writer(&mut vec_len_ref, &vec_len)?;

        Ok(())
    }

    /// Extracts a slice of the data types
    pub fn deserialize_mut_slice<T: Pod>(
        &mut self,
        skip: usize,
        len: usize,
    ) -> Result<&mut [T], ProgramError> {
        let vec_len = self.len();
        let last_item_index = skip
            .checked_add(len)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        if last_item_index > vec_len as usize {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let start_index = VEC_SIZE_BYTES.saturating_add(skip.saturating_mul(mem::size_of::<T>()));
        let end_index = start_index.saturating_add(len.saturating_mul(mem::size_of::<T>()));
        bytemuck::try_cast_slice_mut(&mut self.data[start_index..end_index])
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Extracts a slice of the data types
    pub fn deserialize_slice<T: Pod>(&self, skip: usize, len: usize) -> Result<&[T], ProgramError> {
        let vec_len = self.len();
        let last_item_index = skip
            .checked_add(len)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        if last_item_index > vec_len as usize {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let start_index = VEC_SIZE_BYTES.saturating_add(skip.saturating_mul(mem::size_of::<T>()));
        let end_index = start_index.saturating_add(len.saturating_mul(mem::size_of::<T>()));
        bytemuck::try_cast_slice(&self.data[start_index..end_index])
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Add new element to the end
    pub fn push<T: Pod>(&mut self, element: T) -> Result<(), ProgramError> {
        let mut vec_len_ref = &mut self.data[0..VEC_SIZE_BYTES];
        let mut vec_len = u32::try_from_slice(vec_len_ref)?;

        let start_index = VEC_SIZE_BYTES + vec_len as usize * mem::size_of::<T>();
        let end_index = start_index + mem::size_of::<T>();

        vec_len += 1;
        borsh::to_writer(&mut vec_len_ref, &vec_len)?;

        if self.data.len() < end_index {
            return Err(ProgramError::AccountDataTooSmall);
        }
        let element_ref = bytemuck::try_from_bytes_mut(
            &mut self.data[start_index..start_index + mem::size_of::<T>()],
        )
        .map_err(|_| ProgramError::InvalidAccountData)?;
        *element_ref = element;
        Ok(())
    }

    /// Find matching data in the array
    pub fn find<T: Pod, F: Fn(&[u8]) -> bool>(&self, predicate: F) -> Option<&T> {
        let len = self.len() as usize;
        let mut current = 0;
        let mut current_index = VEC_SIZE_BYTES;
        while current != len {
            let end_index = current_index + mem::size_of::<T>();
            let current_slice = &self.data[current_index..end_index];
            if predicate(current_slice) {
                return Some(bytemuck::from_bytes(current_slice));
            }
            current_index = end_index;
            current += 1;
        }
        None
    }

    /// Find matching data in the array
    pub fn find_mut<T: Pod, F: Fn(&[u8]) -> bool>(&mut self, predicate: F) -> Option<&mut T> {
        let len = self.len() as usize;
        let mut current = 0;
        let mut current_index = VEC_SIZE_BYTES;
        while current != len {
            let end_index = current_index + mem::size_of::<T>();
            let current_slice = &self.data[current_index..end_index];
            if predicate(current_slice) {
                return Some(bytemuck::from_bytes_mut(
                    &mut self.data[current_index..end_index],
                ));
            }
            current_index = end_index;
            current += 1;
        }
        None
    }
}