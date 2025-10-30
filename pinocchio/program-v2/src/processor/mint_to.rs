#[cfg(feature = "logging")]
use pinocchio::log::sol_log;
use {
    super::{shared, unpack_amount},
    pinocchio::{
        account::{AccountCommon as _, AnyAccount, OwnAccountMut},
        account_info::AccountInfo,
        hint::{likely, unlikely},
        program_error::ProgramError,
        pubkey::Pubkey,
        ProgramResult,
    },
    pinocchio_token_interface::{
        error::TokenError,
        state::{account::Account as TokenAccount, mint::Mint},
    },
};

#[inline(always)]
pub fn mint_to(
    program_id: &Pubkey,
    mint: OwnAccountMut<Mint>,
    account: OwnAccountMut<TokenAccount>,
    owner: &AnyAccount,
    amount: u64,
) -> Result<(), ProgramError> {
    #[cfg(feature = "logging")]
    sol_log("Instruction: MintTo");

    let account = account.require_owner_mut(program_id)?.content_mut();
    if unlikely(account.is_frozen()?) {
        return Err(TokenError::AccountFrozen.into());
    }
    if unlikely(account.is_native()) {
        return Err(TokenError::NativeNotSupported.into());
    }

    if unlikely(&account.mint != mint.address()) {
        return Err(TokenError::MintMismatch.into());
    }

    owner.require_signer()?;

    let mint = mint.require_owner_mut(program_id)?.content_mut();

    let Some(mint_authority) = mint.mint_authority() else {
        return Err(TokenError::FixedSupply.into());
    };
    // TODO Multisig support should be added here.
    if unlikely(mint_authority != owner.address()) {
        return Err(TokenError::OwnerMismatch.into());
    }

    if likely(amount != 0) {
        let mint_supply = mint
            .supply()
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;
        mint.set_supply(mint_supply);

        // This should not fail since there is no overflow on the mint supply.
        account.set_amount(account.amount() + amount);
    }

    Ok(())
}

#[inline(always)]
pub fn process_mint_to(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let amount = unpack_amount(instruction_data)?;

    shared::mint_to::process_mint_to(accounts, amount, None)
}
