use anchor_lang::prelude::*;
use anchor_spl::token::*;
use marginfi::{
    program::Marginfi,
    state::{
        marginfi_group::Bank,
        marginfi_account::MarginfiAccount,
    }
};

use crate::state::lending_account::PassiveLendingAccount;

pub const KIVO_MFI_ACCOUNT: &[u8] = b"kivo_mfi_account";

pub fn process(ctx: Context<PassiveLendingAccountRepay>, amount: u64, bump: u8, repay_all: Option<bool>) -> Result<()> {
  let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
  let kivo_signer_seeds = &[&signature_seeds[..]];

  let mfi_repay_acc = marginfi::cpi::accounts::LendingAccountRepay {
      marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
      marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
      signer: ctx.accounts.kivo_account.to_account_info(),
      bank: ctx.accounts.marginfi_bank.to_account_info(),
      signer_token_account: ctx.accounts.kivo_token_account.to_account_info(),
      bank_liquidity_vault: ctx.accounts.bank_vault.to_account_info(),
      token_program: ctx.accounts.token_program.to_account_info(),
  };

  let mfi_repay_ctx = CpiContext::new_with_signer(
      ctx.accounts.marginfi_program.to_account_info().clone(),
      mfi_repay_acc,
      kivo_signer_seeds,
  );

  let repay_all = repay_all;

  let amount = if repay_all == Some(true) {
    0
  } else {
    amount
  };

  marginfi::cpi::lending_account_repay(mfi_repay_ctx, amount, repay_all)?;

  Ok(())
}

#[derive(Accounts)]
pub struct PassiveLendingAccountRepay<'info> {
    /// CHECK: validated by address derivation
    #[account(address = kivo::state::user::User::get_user_address(payer.key()).0)]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            KIVO_MFI_ACCOUNT,
            kivo_account.key().as_ref(),
        ],
        bump,
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,

    /// CHECK: validated by mfi cpi
    pub marginfi_group: UncheckedAccount<'info>,

    pub marginfi_bank: AccountLoader<'info, Bank>,

    /// CHECK: validated by mfi cpi
    #[account(mut)]
    pub bank_vault: UncheckedAccount<'info>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(kivo_account.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account(address = marginfi_bank.load()?.mint)]
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
