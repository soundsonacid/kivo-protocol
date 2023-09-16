use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::state::{
    group::Group,
    group::Balance,
    user::User
};
use crate::error::KivoError;

pub fn process(ctx: Context<Split>, amt: u64) -> Result<()> {

    require!(ctx.accounts.user_balance.balance > amt, KivoError::BadWithdrawal);

    let bump = Group::get_group_address(ctx.accounts.group.admin.key(), ctx.accounts.group.identifier).1;
    let bump_bytes = bytemuck::bytes_of(&bump);
    let identifier_bytes = &ctx.accounts.group.identifier.to_le_bytes();

    let sig_seeds = Group::get_group_signer_seeds(&ctx.accounts.group.admin, identifier_bytes, bump_bytes);
    let signer_seeds = &[&sig_seeds[..]];  

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.group_vault.to_account_info(),
                to: ctx.accounts.destination_vault.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            },
            signer_seeds
        ),
        amt
    )?;

    ctx.accounts.user_balance.decrement_balance(amt);
    ctx.accounts.user_balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct Split<'info> {
    pub group: Account<'info, Group>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = group)]
    pub group_vault: Account<'info, TokenAccount>,

    #[account(address = User::get_user_address(payer.key()).0)]
    pub user: Account<'info, User>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = user)]
    pub destination_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            user.key().as_ref(),
            group.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub user_balance: Account<'info, Balance>,

    pub mint: Account<'info, Mint>,
    
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}