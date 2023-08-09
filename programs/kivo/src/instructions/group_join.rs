use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use crate::{
    state::{
        user::User,
        group::Group,
        group::Invite,
        group::Membership,
    },
    error::KivoError,
    constants::{ INVITE, MEMBERSHIP },
};

pub fn process(ctx: Context<GroupJoin>) -> Result<()> {

    require!(ctx.accounts.group.num_members < 24, KivoError::TooManyGroupMembers);

    ctx.accounts.membership.new(
        ctx.accounts.new_member.key(),
        ctx.accounts.group.key(),
    )?;

    ctx.accounts.group.increment_members();

    Ok(())
}

#[derive(Accounts)]
pub struct GroupJoin<'info> {
    #[account(address = User::get_user_address(payer.key()).0)]
    pub new_member: Account<'info, User>,

    #[account(
        seeds = [
            INVITE,
            new_member.key().as_ref(),
            group.key().as_ref(),
        ],
        bump
    )]
    pub invite: Account<'info, Invite>,

    #[account(mut)]
    pub group: Account<'info, Group>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Membership>(),
        seeds = [
            MEMBERSHIP,
            new_member.key().as_ref(),
            invite.group.as_ref(),
        ],
        bump
    )]
    pub membership: Account<'info, Membership>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}