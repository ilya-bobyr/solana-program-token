// #[cfg(feature = "logging")]
// use pinocchio::log::sol_log;
use {
    super::shared,
    pinocchio::{
        account::{NewAccountMut, SysvarAccount},
        account_info::AccountInfo,
        log::sol_log,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvars::rent::Rent,
        ProgramResult,
    },
    pinocchio_token_interface::{error::TokenError, state::multisig::Multisig},
};

#[inline(always)]
pub fn initialize_multisig(
    _program_id: &Pubkey,
    _multisig: NewAccountMut<Multisig>,
    _rent: &SysvarAccount<Rent>,
    _m: u8,
) -> Result<(), ProgramError> {
    #[cfg(feature = "logging")]
    sol_log("Instruction: InitializeMultisig");

    sol_log("InitializeMultisig is not implemented yet");

    // TODO For some reason, I do not see the transaction failing when this instruction is used.
    // This is incorrect - need to debug it.
    Err(ProgramError::InvalidArgument)
}

#[inline(always)]
pub fn process_initialize_multisig(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let m = instruction_data
        .first()
        .ok_or(TokenError::InvalidInstruction)?;

    shared::initialize_multisig::process_initialize_multisig(accounts, *m, true)
}
