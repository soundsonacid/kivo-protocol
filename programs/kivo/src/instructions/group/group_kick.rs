use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use crate::{
    state::{
        user::User,
        group::Group,
        group::Membership,
    },
    error::KivoError,
    constants::MEMBERSHIP,
};

pub fn process(ctx: Context<KickMemberFromGroup>) -> Result<()> {
    msg!("Kicking user from Group");

    require!(ctx.accounts.group.admin == ctx.accounts.admin.key(), KivoError::NotGroupAdmin);

    ctx.accounts.kicked_membership.close(ctx.accounts.kicked_user.to_account_info())?;

    ctx.accounts.group.decrement_members();

    ctx.accounts.group.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct KickMemberFromGroup<'info> {
    #[account(address = User::get_user_address(payer.key()).0)]
    pub admin: Account<'info, User>,

    pub kicked_user: Account<'info, User>,

    #[account(
        seeds = [
            MEMBERSHIP,
            kicked_user.key().as_ref(),
            group.key().as_ref(),
        ],
        bump
    )]
    pub kicked_membership: Account<'info, Membership>,

    #[account(mut, address = kicked_membership.group)]
    pub group: Account<'info, Group>,

    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}