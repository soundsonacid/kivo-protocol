use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::state::user::User;

pub fn process(ctx: Context<LendingDeposit>, amount: u64) -> Result<()> {
    msg!("Transferring to lender token account");

    let user_bump = User::get_user_address(ctx.accounts.payer.key()).1;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &user_bump);
    let signer_seeds = &[&signature_seeds[..]];
    
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.lender_token_account.to_account_info(),
                authority: ctx.accounts.user_account.to_account_info(),
            },
            signer_seeds
        ),
        amount
    )?;

    msg!("Transferred to lender token account");

    Ok(())
}

#[derive(Accounts)]
pub struct LendingDeposit<'info>{
    pub user_account: Account<'info, User>,

    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: validated by CPI
    pub lending_account: UncheckedAccount<'info>,

    #[account(associated_token::mint = mint, associated_token::authority = lending_account)]
    pub lender_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,
}