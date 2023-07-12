use anchor_lang::prelude::*;

use crate::state::lending_account::PassiveLendingAccount;

pub const LENDING_ACCOUNT: &[u8] = b"passive_lending_account";

#[derive(Accounts)]
pub struct InitializePassiveLendingAccount<'info> {
    /// CHECK: verified by CPI
    pub kivo_account: UncheckedAccount<'info>,

    /// CHECK: awaiting marginfi-cpi
    pub marginfi_account: UncheckedAccount<'info>,

    /// CHECK: awaiting marginfi-cpi
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
    pub lending_account: Account<'info, PassiveLendingAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // pub kivo_program: Program<'info, Kivo>,

    // pub marginfi_program: Program<'info, MarginFi>,

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