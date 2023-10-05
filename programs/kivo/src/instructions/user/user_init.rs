use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use crate::{
    state::user::User,
    constants::USER,
};

pub fn process(ctx: Context<InitializeUser>, name: [u8; 16]) -> Result<()> {
    msg!("Initalizing user!");

    ctx.accounts.user_account.new(
    ctx.accounts.owner.clone().key(),
    name,
    )?;

    ctx.accounts.user_account.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
  #[account(init, 
            payer = payer, 
            space = 8 + std::mem::size_of::<User>(), 
            seeds = [
                USER, 
                payer.key.as_ref()
                ], 
            bump
        )]
  pub user_account: Box<Account<'info, User>>,  

  #[account(mut)]
  pub payer: Signer<'info>,

  pub owner: Signer<'info>,

  #[account(address = system_program::ID)]
  pub system_program: Program<'info, System>

}