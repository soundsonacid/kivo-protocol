use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use anchor_spl::token::TokenAccount;
use crate::{
    state::{
        user::User,
        transaction::Transaction,
    },
    error::KivoError,
};

pub fn process(ctx: Context<RejectRequest>) -> Result<()> {
    msg!("Rejecting request");

    let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
    let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;
    let authority = requester_transaction_account.receiver_account;
    let user = &ctx.accounts.user_account;

    require!(authority == user.key(), KivoError::BadSignerToRejectRequest);

    fulfiller_transaction_account.close(ctx.accounts.user_token_account.to_account_info())?;
    requester_transaction_account.reject();

    requester_transaction_account.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct RejectRequest<'info> {
    pub fulfiller_transaction_account: Account<'info, Transaction>,

    pub requester_transaction_account: Account<'info, Transaction>,

    #[account(address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}