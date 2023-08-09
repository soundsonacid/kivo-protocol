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
    constants::{GROUP, MEMBERSHIP},
};

pub fn process(ctx: Context<TransferGroupOwnership>) -> Result<()> {
    msg!("Transferring group ownership");

    require!(ctx.accounts.group.admin == ctx.accounts.current_admin.key(), KivoError::NotGroupAdmin);

    ctx.accounts.group.transfer_ownership(ctx.accounts.new_admin.key());

    ctx.accounts.group.exit(&crate::id())?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct TransferGroupOwnership<'info> {
    #[account(address = User::get_user_address(payer.key()).0)]
    pub current_admin: Account<'info, User>,

    pub new_admin: Account<'info, User>,

    #[account(
        seeds = [
            MEMBERSHIP,
            new_admin.key().as_ref(),
            group.key().as_ref(),
        ],
        bump
    )]
    pub new_admin_membership: Account<'info, Membership>,

    #[account(
        mut,
        seeds = [
            GROUP,
            current_admin.key().as_ref(),
            &group.group_id.to_le_bytes(),
        ],
        bump
    )]
    pub group: Account<'info, Group>,

    #[account(mt)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
