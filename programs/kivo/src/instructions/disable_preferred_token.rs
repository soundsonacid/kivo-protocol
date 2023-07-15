use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use crate::state::user::User;

pub fn process(ctx: Context<DisablePreferredToken>) -> Result<()> {
    msg!("Disabling preferred token");

    let user = &mut ctx.accounts.user_account;

    user.disable_preferred_token();

    user.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct DisablePreferredToken<'info> {
    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}