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
};

pub fn process(ctx: Context<LeaveGroup>) -> Result<()> {
    msg!("Leaving Group");

    require!(ctx.accounts.member.key() == ctx.accounts.membership.member, KivoError::BadMember);

    ctx.accounts.membership.close(ctx.accounts.payer.to_account_info())?;

    ctx.accounts.group.decrement_members();

    ctx.accounts.group.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct LeaveGroup<'info> {
    pub member: Account<'info, User>,

    #[account(mut)]
    pub group: Account<'info, Group>,

    #[account(mut)]
    pub membership: Account<'info, Membership>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}