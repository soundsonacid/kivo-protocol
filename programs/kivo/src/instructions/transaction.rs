use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, sysvar};
use anchor_spl::token::*;
use std::mem::size_of;

use crate::state::transaction::Transaction;
use crate::state::user::User;

#[derive(Accounts)]
pub struct CreateTransactionAccount<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Transaction>(), // pk + pk + u64 + u64 + pk + bool
        seeds = [
            b"transaction",
            user_account.to_account_info().key.as_ref(),
            user_account.payments_sent.to_le_bytes().as_ref()],
        bump
    )]
    pub user_transaction_account: Account<'info, Transaction>,

    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Transaction>(), // pk + u16 + u64 + u64 + pk + bool
        seeds = [
            b"transaction",
            receiver_account.to_account_info().key.as_ref(),
            receiver_account.payments_received.to_le_bytes().as_ref()],
        bump
    )]
    pub receiver_transaction_account: Account<'info, Transaction>,

    #[account(
        mut,
        seeds = [
            b"user",
            payer.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub receiver_account: Account<'info, User>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    /// CHECK: validated by cpi signer seeds
    pub sender: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"user",
            payer.key().as_ref(),
        ],
        bump
    )]
    pub sender_user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = sender_user_account, associated_token::mint = mint)]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub receiver_user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = receiver_user_account, associated_token::mint = mint)]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}