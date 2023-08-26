use anchor_lang::{
    prelude::*,
    solana_program::{
        system_program,
        sysvar
    }
};
use anchor_spl::token::*;
use crate::{
    constants::{
        INCOMING,
        OUTGOING
    },
    state::{
        user::User,
        transaction::Transaction,
    }
};

pub fn process(ctx: Context<CreateRequest>, amount: u64 )-> Result<()> {
    msg!("Creating request");

    let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;
    let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
    let requester = &mut ctx.accounts.requester;
    let fulfiller = &mut ctx.accounts.fulfiller;

    requester_transaction_account.new(
        requester.key(),
        fulfiller.key(),
        amount,
        None
    )?;

    requester_transaction_account.exit(&crate::id())?;

    fulfiller_transaction_account.new(
        requester.key(),
        fulfiller.key(),
        amount,
        None
    )?;

    fulfiller_transaction_account.exit(&crate::id())?;

    requester.increment_incoming_transactions();
    fulfiller.increment_outgoing_transactions();

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
            OUTGOING,
            requester.to_account_info().key.as_ref(),
            requester.outgoing_tx.to_le_bytes().as_ref()],
        bump
    )]
    pub requester_transaction_account: Account<'info, Transaction>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            INCOMING,
            fulfiller.to_account_info().key.as_ref(),
            fulfiller.incoming_tx.to_le_bytes().as_ref()],
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