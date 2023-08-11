use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::token::*;
use crate::state::user::User;

pub fn process(ctx: Context<SetPreferredToken>) -> Result<()> {
    msg!("Setting preferred token");

    let user = &mut ctx.accounts.user_account;
    let new_preferred_token = &ctx.accounts.preferred_token_mint;

    user.set_preferred_token(new_preferred_token.key());

    user.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct SetPreferredToken<'info> {
    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account()]
    pub preferred_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}