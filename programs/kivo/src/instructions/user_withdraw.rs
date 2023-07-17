use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::token::*;
use crate::state::user::User;

pub fn process(ctx: Context<Withdrawal>, amount: u64) -> Result<()> {
    msg!("Withdrawing");

    let bump = User::get_user_address(ctx.accounts.payer.key()).1;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];    

    let token_program = ctx.accounts.token_program.to_account_info().clone();

    let transfer_accounts = Transfer {
        from: ctx.accounts.pda_token_account.to_account_info(),
        to: ctx.accounts.withdrawer_token_account.to_account_info(),
        authority: ctx.accounts.user_account.to_account_info().clone(),
    };
    let cpi_ctx_transfer = CpiContext::new_with_signer(
        token_program.to_account_info().clone(),
        transfer_accounts,
        signer_seeds,
    );
    transfer(cpi_ctx_transfer, amount)?;

    ctx.accounts.user_account.increment_withdrawals();

    ctx.accounts.user_account.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    /// CHECK: Validated by signer seeds
    pub withdrawer: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = withdrawer, associated_token::mint = mint)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

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