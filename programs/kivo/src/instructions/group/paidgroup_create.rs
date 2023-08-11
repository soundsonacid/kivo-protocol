use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use crate::{
    state::{
        user::User,
        group::PaidGroup,
        group::Membership,
    },
    constants::{PAID, MEMBERSHIP},
};

pub fn process(ctx: Context<CreatePaidGroup>, group_id: u32, group_name: [u8; 32], fee: u64, recurring: bool) -> Result<()> {
    ctx.accounts.group.new(
        group_id,
        group_name,
        ctx.accounts.group_admin.key(),
        fee,
        recurring
    )?;

    ctx.accounts.group.increment_members();

    ctx.accounts.group.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(group_id: u32) ]
pub struct CreatePaidGroup<'info> {
    #[account(address = User::get_user_address(payer.key()).0)]
    pub group_admin: Account<'info, User>,

    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<PaidGroup>() + 8,
        seeds = [
            PAID,
            group_admin.to_account_info().key.as_ref(),
            &group_id.to_le_bytes(),
        ],
        bump
    )]
    pub group: Account<'info, PaidGroup>,
    
    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<Membership>() + 8,
        seeds = [
            MEMBERSHIP,
            group_admin.key().as_ref(),
            group.key().as_ref(),
        ],
        bump
    )]
    pub membership: Account<'info, Membership>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}