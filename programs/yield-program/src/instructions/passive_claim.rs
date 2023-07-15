use anchor_lang::prelude::*;
use anchor_spl::token::*;
use marginfi::program::Marginfi;

use crate::state::lending_account::PassiveLendingAccount;

pub fn handler(_ctx: Context<PassiveLendingAccountClaim>, _amount: u64, _bump: u8) -> Result<()> {

    Ok(())
}

#[derive(Accounts)]
pub struct PassiveLendingAccountClaim<'info> {
    /// CHECK: validated by address derivation
    #[account(address = kivo::state::user::User::get_user_address(payer.key()).0)]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(payer.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated by mfi cpi
    pub marginfi_program: Program<'info, Marginfi>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,
}
