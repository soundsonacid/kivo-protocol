use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::token::*;
use crate::state::user::User;

pub fn process(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    msg!("Depositing");

    let deposit_accounts = Transfer {
        from: ctx.accounts.depositor_token_account.to_account_info(),
        to: ctx.accounts.pda_token_account.to_account_info(),
        authority: ctx.accounts.depositor.to_account_info().clone(),
    };

    let token_program = ctx.accounts.token_program.to_account_info().clone();

    let deposit_cpi_context = CpiContext::new(token_program, deposit_accounts);

    transfer(deposit_cpi_context, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: validated by cpi
    #[account(address = payer.key())]
    pub depositor: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = depositor, associated_token::mint = mint)]
    pub depositor_token_account: Account<'info, TokenAccount>,

    /// CHECK: validated by address derivation
    #[account(address = User::get_user_address(payer.key()).0)]
    pub user_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub pda_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}