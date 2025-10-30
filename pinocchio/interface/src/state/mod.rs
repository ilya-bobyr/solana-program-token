use core::ptr::addr_of_mut;

use pinocchio::program_error::ProgramError;

pub mod account;
pub mod account_state;
pub mod mint;
pub mod multisig;

/// Type alias for fields represented as `COption`.
// TODO This should be a proper type, not an alias :'(
pub type COption<T> = ([u8; 4], T);

/// # Safety
///
/// `uninit` must be zero initialized, not uninitialized completely.
#[inline(always)]
pub unsafe fn set_coption_some<T>(uninit: *mut COption<T>, value: T) {
    // SAFETY We are only writing though the potentially uninitialized pointer.
    unsafe { addr_of_mut!((*uninit).0[0]).write(1) }
    // SAFETY We are only writing though the potentially uninitialized pointer.
    unsafe { addr_of_mut!((*uninit).1).write(value) }
}

/// # Safety
///
/// `uninit` must be zero initialized, not uninitialized completely.
#[inline(always)]
pub unsafe fn set_coption<T>(uninit: *mut COption<T>, value: Option<T>) {
    match value {
        Some(value) => unsafe { set_coption_some(uninit, value) },
        None => (),
    }
}

/// Marker trait for types that can be cast from a raw pointer.
///
/// # Safety
///
/// It is up to the type implementing this trait to guarantee that the cast is
/// safe, i.e., the fields of the type are well aligned and there are no padding
/// bytes.
pub unsafe trait Transmutable {
    /// The length of the type.
    ///
    /// This must be equal to the size of each individual field in the type.
    const LEN: usize;
}

/// Trait to represent a type that can be initialized.
pub trait Initializable {
    /// Return `true` if the object is initialized.
    fn is_initialized(&self) -> Result<bool, ProgramError>;
}

/// Return a reference for an initialized `T` from the given bytes.
///
/// # Safety
///
/// The caller must ensure that `bytes` contains a valid representation of `T`.
#[inline(always)]
pub unsafe fn load<T: Initializable + Transmutable>(bytes: &[u8]) -> Result<&T, ProgramError> {
    load_unchecked(bytes).and_then(|t: &T| {
        // checks if the data is initialized
        if t.is_initialized()? {
            Ok(t)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    })
}

/// Return a `T` reference from the given bytes.
///
/// This function does not check if the data is initialized.
///
/// # Safety
///
/// The caller must ensure that `bytes` contains a valid representation of `T`.
#[inline(always)]
pub unsafe fn load_unchecked<T: Transmutable>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

/// Return a mutable reference for an initialized `T` from the given bytes.
///
/// # Safety
///
/// The caller must ensure that `bytes` contains a valid representation of `T`.
#[inline(always)]
pub unsafe fn load_mut<T: Initializable + Transmutable>(
    bytes: &mut [u8],
) -> Result<&mut T, ProgramError> {
    load_mut_unchecked(bytes).and_then(|t: &mut T| {
        // checks if the data is initialized
        if t.is_initialized()? {
            Ok(t)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    })
}

/// Return a mutable `T` reference from the given bytes.
///
/// This function does not check if the data is initialized.
///
/// # Safety
///
/// The caller must ensure that `bytes` contains a valid representation of `T`.
#[inline(always)]
pub unsafe fn load_mut_unchecked<T: Transmutable>(
    bytes: &mut [u8],
) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}

#[cfg(test)]
mod tests {
    use super::{account::Account, mint::Mint, multisig::Multisig};

    #[test]
    fn all_account_sizes_are_distinct() {
        let mint_size = size_of::<Mint>();
        let account_size = size_of::<Account>();
        let multisig_size = size_of::<Multisig>();

        assert_ne!(
            mint_size, account_size,
            "`Mint` and `Account` sizes must me distinct"
        );
        assert_ne!(
            mint_size, multisig_size,
            "`Mint` and `Multisig` sizes must me distinct"
        );
        assert_ne!(
            account_size, multisig_size,
            "`Account` and `Multisig` sizes must me distinct"
        );
    }
}
