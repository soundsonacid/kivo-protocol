use anchor_lang::{
    prelude::*,
    solana_program::system_program
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
    },
    error::KivoError,
};

pub fn process(ctx: Context<ExecuteTransaction>, amount: u64) -> Result<()> {
    msg!("Executing transaction");

    let sender_transaction_account = &mut ctx.accounts.sender_transaction_account;
    let receiver_transaction_account = &mut ctx.accounts.receiver_transaction_account;
    let sender = &mut ctx.accounts.sender_user_account;
    let receiver = &mut ctx.accounts.receiver_user_account;

    let bump = User::get_user_address(ctx.accounts.payer.key()).1;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];

    let fee = amount / 400;

    let amt_final = amount - fee;

    require!(amt_final > 0, KivoError::NegDelta);
    
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.kivo_vault.to_account_info(),
                authority: sender.to_account_info().clone(),
            },
            signer_seeds,
        ),
        fee
    )?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: sender.to_account_info().clone(),
            },
            signer_seeds,
        ),
        amt_final
    )?;

    sender_transaction_account.new(
        receiver.key(),
        sender.key(),
        amount,
        Some(true),
    )?;

    receiver_transaction_account.new(
        receiver.key(),
        sender.key(),
        amount,
        Some(true)
    )?;

    receiver.increment_incoming_transactions();
    sender.increment_outgoing_transactions();

    receiver.exit(&crate::id())?;
    sender.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub sender_user_account: Box<Account<'info, User>>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            OUTGOING,
            sender_user_account.to_account_info().key.as_ref(),
            sender_user_account.outgoing_tx.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub sender_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = sender_user_account, associated_token::mint = mint)]
    pub sender_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub receiver_user_account: Account<'info, User>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            INCOMING,
            receiver_user_account.to_account_info().key.as_ref(),
            receiver_user_account.incoming_tx.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub receiver_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = receiver_user_account, associated_token::mint = mint)]
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub kivo_vault: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}