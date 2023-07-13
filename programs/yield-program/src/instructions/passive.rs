use anchor_lang::prelude::*;
// use marginfi::state::marginfi_account::MarginfiAccount;
// use marginfi::state::marginfi_group::MarginfiGroup;
use marginfi::program::Marginfi;
// use kivo::state::user::User;

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

// #[derive(Accounts)]
// pub struct PassiveLendingDeposit<'info> {
    // pub kivo_account: UncheckedAccount<'info>,

    // pub marginfi_account: UncheckedAccount<'info>,

    // pub kivo_program: Program<'info, Kivo>,

    // pub marginfi_program: Program<'info, MarginFi>,

    // pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct PassiveLendingWithdraw<'info> {

// }