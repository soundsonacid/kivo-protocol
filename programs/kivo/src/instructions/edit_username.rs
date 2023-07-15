use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use crate::{
    state::user::{User, Username},
    constants::USERNAME,
    error::KivoError,
};

pub fn process(ctx: Context<EditUsername>, name: [u8; 16]) -> Result<()> {
    msg!("Editing username");

    require!(name.iter().all(|&value| (value >= 97 && value <= 122) || (value >= 48 && value <= 57) || (value == 0)), KivoError::InvalidUsername);

    let new_username = &mut ctx.accounts.new_username_account;
    let user = &mut ctx.accounts.user_account;
    
    ctx.accounts.old_username_account.close(user.to_account_info())?;

    new_username.new(
        user.key(),
        name,
    )?;

    user.set_username(name);

    user.exit(&crate::id())?;
    new_username.exit(&crate::id())?;
    
    Ok(())
}


#[derive(Accounts)]
#[instruction(new_name: [u8; 16])]
pub struct EditUsername<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Username>(),
        seeds = [
            USERNAME, 
            new_name.as_ref()
        ],
        bump,
    )]
    pub new_username_account: Account<'info, Username>,

    #[account(
        mut,
        has_one = user_account,
    )]
    pub old_username_account: Account<'info, Username>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}