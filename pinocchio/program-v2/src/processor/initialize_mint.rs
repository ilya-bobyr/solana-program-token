#[cfg(feature = "logging")]
use pinocchio::log::sol_log;
use {
    pinocchio::{
        account::{AccountCommon as _, NewAccountMut, SysvarAccount},
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvars::rent::Rent,
    },
    pinocchio_token_interface::{
        error::TokenError,
        state::mint::{Mint, MintInit},
    },
};

#[inline(always)]
pub fn initialize_mint(
    _program_id: &Pubkey,
    mint: NewAccountMut<Mint>,
    rent: &SysvarAccount<Rent>,
    decimals: u8,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
) -> Result<(), ProgramError> {
    #[cfg(feature = "logging")]
    sol_log("Instruction: InitializeMint");

    let rent = rent.content();
    if !rent.is_exempt(mint.lamports(), mint.data_len() as usize) {
        return Err(TokenError::NotRentExempt.into());
    }

    let _ = mint.initialize(MintInit {
        mint_authority,
        decimals,
        freeze_authority,
    })?;

    Ok(())
}
