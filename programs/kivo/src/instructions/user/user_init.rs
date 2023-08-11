use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use crate::{
    state::user::{User, Username},
    constants::{USER, USERNAME},
    error::KivoError,
};

pub fn process(ctx: Context<InitializeUser>, name: [u8; 16], account_type: u8) -> Result<()> {
    msg!("Initalizing user!");

    require!(name.iter().all(|&value| (value >= 97 && value <= 122) || (value >= 48 && value <= 57) || (value == 0)), KivoError::InvalidUsername);

    let user = &mut ctx.accounts.user_account;
    let username = &mut ctx.accounts.username_account;

    username.new(
    user.key(),
    name,
    )?;

    user.new(
    ctx.accounts.owner.clone().key(),
    name,
    account_type,
    )?;

    username.exit(&crate::id())?;
    user.exit(&crate::id())?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(name: [u8; 16])]
pub struct InitializeUser<'info> {

  #[account(init, 
            payer = payer, 
            space = 8 + std::mem::size_of::<Username>(), 
            seeds = [
                USERNAME, 
                name.as_ref()
                ], 
            bump
        )]
  pub username_account: Box<Account<'info, Username>>,

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