use {
    super::{shared, unpack_amount},
    pinocchio::{
        account::{AnyAccount, OwnAccountMut},
        account_info::AccountInfo,
        log::sol_log,
        program_error::ProgramError,
        pubkey::Pubkey,
        ProgramResult,
    },
    pinocchio_token_interface::state::account::Account as TokenAccount,
};

pub fn transfer(
    _program_id: &Pubkey,
    _source: OwnAccountMut<TokenAccount>,
    _destination: OwnAccountMut<TokenAccount>,
    _owner: &AnyAccount,
    _amount: u64,
) -> Result<(), ProgramError> {
    #[cfg(feature = "logging")]
    sol_log("Instruction: Transfer");

    sol_log("Transfer is not implemented yet");

    // TODO For some reason, I do not see the transaction failing when this instruction is used.
    // This is incorrect - need to debug it.
    Err(ProgramError::InvalidArgument)
}

#[inline(always)]
pub fn process_transfer(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let amount = unpack_amount(instruction_data)?;

    shared::transfer::process_transfer(accounts, amount, None)
}
