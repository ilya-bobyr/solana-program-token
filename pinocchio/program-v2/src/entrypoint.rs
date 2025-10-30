use {
    crate::processor::{
        initialize_account, initialize_mint, initialize_multisig, mint_to, transfer,
    },
    pinocchio::{
        account::{AnyAccount, NewAccountMut, OwnAccount, OwnAccountMut, SysvarAccount},
        log::{sol_log, sol_log_64},
        no_allocator, nostd_panic_handler,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvars::rent::Rent,
    },
    pinocchio_macro::entrypoint,
    pinocchio_token_interface::{
        instruction::TokenInstruction,
        state::{account::Account as TokenAccount, mint::Mint, multisig::Multisig},
    },
};

// Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
nostd_panic_handler!();

entrypoint! {
    instruction_type: TokenInstruction,
    program_id: program_id;

    TokenInstruction::InitializeMint => |
        mint: NewAccountMut<Mint>,
        rent: &SysvarAccount<Rent>,
        ;
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    |,

    TokenInstruction::InitializeAccount => |
        account: NewAccountMut<TokenAccount>,
        mint: &OwnAccount<Mint>,
        owner: &AnyAccount,
        rent: &SysvarAccount<Rent>,
    |,

    TokenInstruction::InitializeMultisig => |
        multisig: NewAccountMut<Multisig>,
        rent: &SysvarAccount<Rent>,
        /* TODO: signers: &[AnyAccount], */
        ;
        m: u8,
    |,

    TokenInstruction::Transfer => |
        source: OwnAccountMut<TokenAccount>,
        destination: OwnAccountMut<TokenAccount>,
        owner: &AnyAccount,
        ;
        amount: u64,
    |,

    TokenInstruction::MintTo => |
        mint: OwnAccountMut<Mint>,
        account: OwnAccountMut<TokenAccount>,
        owner: &AnyAccount,
        ;
        amount: u64,
    |,

    inst @ _ => || {
        sol_log("Instruction not implemented yet");
        sol_log_64(0, 0, 0, 0, inst as u64);
        Err(ProgramError::InvalidArgument)
    },
}

//- /// Log an error.
//- #[cold]
//- fn log_error(error: &ProgramError) {
//-     sol_log(error.to_str::<TokenError>());
//- }

//- /// Process an instruction.
//- ///
//- /// In the first stage, the entrypoint checks the discriminator of the
//- /// instruction data to determine whether the instruction is a "batch"
//- /// instruction or a "regular" instruction. This avoids nesting of "batch"
//- /// instructions, since it is not sound to have a "batch" instruction inside
//- /// another "batch" instruction.
//- #[inline(always)]
//- pub fn process_instruction(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
//-     let [discriminator, remaining @ ..] = instruction_data else {
//-         return Err(TokenError::InvalidInstruction.into());
//-     };
//-
//-     let result = if *discriminator == 255 {
//-         // 255 - Batch
//-         #[cfg(feature = "logging")]
//-         pinocchio::msg!("Instruction: Batch");
//-
//-         process_batch(accounts, remaining)
//-     } else {
//-         inner_process_instruction(accounts, instruction_data)
//-     };
//-
//-     result.inspect_err(log_error)
//- }

//- /// Process a "regular" instruction.
//- ///
//- /// The processor of the token program is divided into two parts to reduce the
//- /// overhead of having a large `match` statement. The first part of the
//- /// processor handles the most common instructions, while the second part
//- /// handles the remaining instructions.
//- ///
//- /// The rationale is to reduce the overhead of making multiple comparisons for
//- /// popular instructions.
//- ///
//- /// Instructions on the first part of the inner processor:
//- ///
//- /// - `0`: `InitializeMint`
//- /// - `1`: `InitializeAccount`
//- /// - `3`: `Transfer`
//- /// - `7`: `MintTo`
//- /// - `9`: `CloseAccount`
//- /// - `16`: `InitializeAccount2`
//- /// - `18`: `InitializeAccount3`
//- /// - `20`: `InitializeMint2`
//- #[inline(always)]
//- pub(crate) fn inner_process_instruction(
//-     accounts: &[AccountInfo],
//-     instruction_data: &[u8],
//- ) -> ProgramResult {
//-     let [discriminator, instruction_data @ ..] = instruction_data else {
//-         return Err(TokenError::InvalidInstruction.into());
//-     };
//-
//-     match *discriminator {
//-         // 0 - InitializeMint
//-         0 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeMint");
//-
//-             process_initialize_mint(accounts, instruction_data)
//-         }
//-         // 1 - InitializeAccount
//-         1 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeAccount");
//-
//-             process_initialize_account(accounts)
//-         }
//-         // 3 - Transfer
//-         3 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: Transfer");
//-
//-             process_transfer(accounts, instruction_data)
//-         }
//-         // 7 - MintTo
//-         7 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: MintTo");
//-
//-             process_mint_to(accounts, instruction_data)
//-         }
//-         // 8 - Burn
//-         8 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: Burn");
//-
//-             process_burn(accounts, instruction_data)
//-         }
//-         // 9 - CloseAccount
//-         9 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: CloseAccount");
//-
//-             process_close_account(accounts)
//-         }
//-         // 12 - TransferChecked
//-         12 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: TransferChecked");
//-
//-             process_transfer_checked(accounts, instruction_data)
//-         }
//-         // 15 - BurnChecked
//-         15 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: BurnChecked");
//-
//-             process_burn_checked(accounts, instruction_data)
//-         }
//-         // 17 - SyncNative
//-         17 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: SyncNative");
//-
//-             process_sync_native(accounts)
//-         }
//-         // 18 - InitializeAccount3
//-         18 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeAccount3");
//-
//-             process_initialize_account3(accounts, instruction_data)
//-         }
//-         // 20 - InitializeMint2
//-         20 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeMint2");
//-
//-             process_initialize_mint2(accounts, instruction_data)
//-         }
//-         // 22 - InitializeImmutableOwner
//-         22 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeImmutableOwner");
//-
//-             process_initialize_immutable_owner(accounts)
//-         }
//-         d => inner_process_remaining_instruction(accounts, instruction_data, d),
//-     }
//- }

//- /// Process a remaining "regular" instruction.
//- ///
//- /// This function is called by the [`inner_process_instruction`] function if the
//- /// discriminator does not match any of the common instructions. This function
//- /// is used to reduce the overhead of having a large `match` statement in the
//- /// [`inner_process_instruction`] function.
//- fn inner_process_remaining_instruction(
//-     accounts: &[AccountInfo],
//-     instruction_data: &[u8],
//-     discriminator: u8,
//- ) -> ProgramResult {
//-     match discriminator {
//-         // 2 - InitializeMultisig
//-         2 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeMultisig");
//-
//-             process_initialize_multisig(accounts, instruction_data)
//-         }
//-         // 4 - Approve
//-         4 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: Approve");
//-
//-             process_approve(accounts, instruction_data)
//-         }
//-         // 5 - Revoke
//-         5 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: Revoke");
//-
//-             process_revoke(accounts)
//-         }
//-         // 6 - SetAuthority
//-         6 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: SetAuthority");
//-
//-             process_set_authority(accounts, instruction_data)
//-         }
//-         // 10 - FreezeAccount
//-         10 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: FreezeAccount");
//-
//-             process_freeze_account(accounts)
//-         }
//-         // 11 - ThawAccount
//-         11 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: ThawAccount");
//-
//-             process_thaw_account(accounts)
//-         }
//-         // 13 - ApproveChecked
//-         13 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: ApproveChecked");
//-
//-             process_approve_checked(accounts, instruction_data)
//-         }
//-         // 14 - MintToChecked
//-         14 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: MintToChecked");
//-
//-             process_mint_to_checked(accounts, instruction_data)
//-         }
//-         // 16 - InitializeAccount2
//-         16 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeAccount2");
//-
//-             process_initialize_account2(accounts, instruction_data)
//-         }
//-         // 19 - InitializeMultisig2
//-         19 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: InitializeMultisig2");
//-
//-             process_initialize_multisig2(accounts, instruction_data)
//-         }
//-         // 21 - GetAccountDataSize
//-         21 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: GetAccountDataSize");
//-
//-             process_get_account_data_size(accounts)
//-         }
//-         // 23 - AmountToUiAmount
//-         23 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: AmountToUiAmount");
//-
//-             process_amount_to_ui_amount(accounts, instruction_data)
//-         }
//-         // 24 - UiAmountToAmount
//-         24 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: UiAmountToAmount");
//-
//-             process_ui_amount_to_amount(accounts, instruction_data)
//-         }
//-         // 38 - WithdrawExcessLamports
//-         38 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: WithdrawExcessLamports");
//-
//-             process_withdraw_excess_lamports(accounts)
//-         }
//-         // 45 - UnwrapLamports
//-         45 => {
//-             #[cfg(feature = "logging")]
//-             pinocchio::msg!("Instruction: UnwrapLamports");
//-
//-             process_unwrap_lamports(accounts, instruction_data)
//-         }
//-         _ => Err(TokenError::InvalidInstruction.into()),
//-     }
//- }
