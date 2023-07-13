use anchor_lang::prelude::*;
use anchor_spl::token::*;
use marginfi::program::Marginfi;

use crate::state::lending_account::PassiveLendingAccount;

pub const LENDING_ACCOUNT: &[u8] = b"passive_lending_account";
pub const USER: &[u8] = b"user";
pub const KIVO_MFI_ACCOUNT: &[u8] = b"kivo_mfi_account";

#[derive(Accounts)]
pub struct InitializePassiveLendingAccount<'info> {
    /// CHECK: verified by pda derivation
    #[account(
        seeds = [
            USER,
            payer.key().as_ref(),
        ],
        bump,
    )]
    pub kivo_account: UncheckedAccount<'info>,

    /// CHECK: verified by mfi cpi call
    #[account(
        mut,
        seeds = [
            KIVO_MFI_ACCOUNT,
            kivo_account.key().as_ref(),
        ],
        bump,
    )]
    pub marginfi_account: UncheckedAccount<'info>,

    /// CHECK: verified by mfi cpi call
    pub marginfi_group: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<PassiveLendingAccount>(),
        seeds = [
            LENDING_ACCOUNT,
            kivo_account.key().as_ref()
        ],
        bump,
    )]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub marginfi_program: Program<'info, Marginfi>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassiveLendingAccountDeposit<'info> {
    /// CHECK: verified by pda derivation
    #[account(
        seeds = [
            USER,
            payer.key().as_ref(),
        ],
        bump,
    )]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(payer.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub marginfi_program: Program<'info, Marginfi>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassiveLendingAccountWithdraw<'info> {
    /// CHECK: verified by pda derivation
    #[account(
        seeds = [
            USER,
            payer.key().as_ref(),
        ],
        bump,
    )]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(payer.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub marginfi_program: Program<'info, Marginfi>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassiveLendingAccountClaimInterest<'info> {
    /// CHECK: verified by pda derivation
    #[account(
        seeds = [
            USER,
            payer.key().as_ref(),
        ],
        bump,
    )]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(payer.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub marginfi_program: Program<'info, Marginfi>,

    pub system_program: Program<'info, System>,
}