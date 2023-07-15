use anchor_lang::{
    prelude::*,
    solana_program::{
        system_program,
        sysvar
    }
};
use anchor_spl::token::*;
use crate::{
    constants::TRANSACTION,
    state::{
        user::User,
        transaction::Transaction,
    }
};

pub fn process(ctx: Context<CreateRequest>, amount: u64, time_stamp: u64) -> Result<()> {
    msg!("Creating request");

    let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;
    let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
    let requester = &mut ctx.accounts.requester;
    let fulfiller = &mut ctx.accounts.fulfiller;
    let mint = &ctx.accounts.mint;

    requester_transaction_account.new(
        requester.key(),
        requester.username.clone(),
        mint.key(), 
        amount, 
        time_stamp, 
        fulfiller.key(),
        fulfiller.username.clone(),
        fulfiller_transaction_account.key(), 
        false, 
    )?;

    requester_transaction_account.exit(&crate::id())?;

    let fulfiller_transaction_account_key = fulfiller_transaction_account.key();

    fulfiller_transaction_account.new(
        requester.key(),
        requester.username.clone(),
        mint.key(),
        amount,
        time_stamp,
        fulfiller.key(),
        fulfiller.username.clone(),
        fulfiller_transaction_account_key,
        false,
    )?;

    fulfiller_transaction_account.exit(&crate::id())?;

    requester.increment_transactions();
    fulfiller.increment_transactions();

    requester.exit(&crate::id())?;
    fulfiller.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateRequest<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            TRANSACTION,
            requester.to_account_info().key.as_ref(),
            requester.transactions.to_le_bytes().as_ref()],
        bump
    )]
    pub requester_transaction_account: Account<'info, Transaction>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            TRANSACTION,
            fulfiller.to_account_info().key.as_ref(),
            fulfiller.transactions.to_le_bytes().as_ref()],
        bump
    )]
    pub fulfiller_transaction_account: Account<'info, Transaction>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub requester: Account<'info, User>,

    #[account(mut)]
    pub fulfiller: Account<'info, User>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}