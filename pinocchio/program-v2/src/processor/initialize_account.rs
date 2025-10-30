#[cfg(feature = "logging")]
use pinocchio::log::sol_log;
use {
    pinocchio::{
        account::{AccountCommon as _, AnyAccount, NewAccountMut, OwnAccount, SysvarAccount},
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvars::rent::Rent,
    },
    pinocchio_token_interface::{
        error::TokenError,
        state::{
            account::{Account as TokenAccount, AccountInit},
            mint::Mint,
        },
    },
};

#[inline(always)]
pub fn initialize_account(
    _program_id: &Pubkey,
    account: NewAccountMut<TokenAccount>,
    mint: &OwnAccount<Mint>,
    owner: &AnyAccount,
    rent: &SysvarAccount<Rent>,
) -> Result<(), ProgramError> {
    #[cfg(feature = "logging")]
    sol_log("Instruction: InitializeAccount");

    // TODO This instruction needs to support "native mint" - it is a special case, when the `mint`
    // account is an account with `pinocchio_token_interface::native_mint::ID`.
    //
    // What is the best way to express that?  One options seems to provide downcasts from
    // `AnyAccount`.  `mint` can be then initially `AnyAccount`, check if `address()` is of the
    // `native_mint`.  If not then downcast to `OwnAccount<Mint>` with corresponding checks?

    let rent = rent.content();
    if !rent.is_exempt(account.lamports(), account.data_len() as usize) {
        return Err(TokenError::NotRentExempt.into());
    }

    let _ = account.initialize(AccountInit {
        mint: mint.address(),
        owner: owner.address(),
    })?;

    Ok(())
}
