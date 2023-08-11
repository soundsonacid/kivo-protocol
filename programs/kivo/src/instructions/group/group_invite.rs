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

pub fn process(ctx: Context<GroupInvite>) -> Result<()> {
    msg!("Inviting user to Group");

    require!(ctx.accounts.group.num_members < 24, KivoError::TooManyGroupMembers);

    ctx.accounts.invite.new(
        ctx.accounts.invitee.key(),
        ctx.accounts.group.key(),
    )?;

    ctx.accounts.invite.exit(&crate::id())?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct GroupInvite<'info> {
    pub invitee: Account<'info, User>,
    
    #[account(address = User::get_user_address(payer.key()).0)]
    pub invitor: Account<'info, User>,

    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<Invite>() + 8,
        seeds = [
            INVITE,
            invitee.key().as_ref(),
            membership.group.as_ref(),
        ],
        bump
    )]
    pub invite: Account<'info, Invite>,

    #[account(
        seeds = [
            MEMBERSHIP,
            invitor.key().as_ref(),
            group.key().as_ref(),
        ],
        bump
    )]
    pub membership: Account<'info, Membership>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = membership.group)]
    pub group: Account<'info, Group>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}