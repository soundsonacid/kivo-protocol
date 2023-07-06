use anchor_lang::prelude::*;

use crate::state::lending_account::LendingAccount;

pub const LENDING_ACCOUNT: &[u8] = b"lending_account";

#[derive(Accounts)]
pub struct InitializeLendingAccount<'info> {
    /// CHECK: verified by CPI
    pub kivo_account: UncheckedAccount<'info>,

    /// CHECK: verified by CPI
    pub marginfi_account: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<LendingAccount>(),
        seeds = [
            LENDING_ACCOUNT,
            kivo_account.key().as_ref()
        ],
        bump,
    )]
    pub lending_account: Account<'info, LendingAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}