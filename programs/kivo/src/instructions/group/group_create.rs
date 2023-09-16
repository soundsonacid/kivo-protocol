use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use crate::{
    state::{
        user::User,
        group::Group,
    },
    constants::GROUP,
};

pub fn process(ctx: Context<CreateGroup>, group_id: u32, group_name: [u8; 32]) -> Result<()> {
    ctx.accounts.group.new(
        group_id,
        group_name,
        ctx.accounts.group_admin.key(),
        ctx.accounts.group_admin.num_groups as u8,
    )?;

    ctx.accounts.group.increment_members();

    ctx.accounts.group_admin.increment_groups();

    ctx.accounts.group.exit(&crate::id())?;
    ctx.accounts.group_admin.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateGroup<'info> {
    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub group_admin: Account<'info, User>,

    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<Group>() + 8,
        seeds = [
            GROUP,
            group_admin.to_account_info().key.as_ref(),
            &group_admin.num_groups.to_le_bytes(),
        ],
        bump
    )]
    pub group: Account<'info, Group>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}