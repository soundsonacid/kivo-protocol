use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};

use crate::{
    state::user::{User, Friend},
    constants::FRIEND,
};

pub fn process(ctx: Context<AddFriend>) -> Result<()> {
    msg!("Adding friend");

    let user = &mut ctx.accounts.user_account;
    let friend = &ctx.accounts.friend_account;
    let friend_account = &mut ctx.accounts.new_friend;

    friend_account.new(
        friend.key(),
        friend.username.clone(),
        friend.account_type.clone(),
    )?;

    user.increment_friends();
    user.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct AddFriend<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Friend>(),
        seeds = [
            FRIEND,
            user_account.to_account_info().key.as_ref(),
            user_account.num_friends.to_le_bytes().as_ref()
        ],        
        bump
    )]
    pub new_friend: Account<'info, Friend>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    pub friend_account: Account<'info, User>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}