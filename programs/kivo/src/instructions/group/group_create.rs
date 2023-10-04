use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use crate::state::{
    user::User,
    group::Group,
};

pub fn process(ctx: Context<CreateGroup>) -> Result<()> {
    msg!("Initializing group {}", ctx.accounts.group.key().to_string());

    ctx.accounts.group.new(
        ctx.accounts.group_admin.key(),
        ctx.accounts.group_admin.num_groups as u8,
    )?;

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
    )]
    pub group: Account<'info, Group>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}