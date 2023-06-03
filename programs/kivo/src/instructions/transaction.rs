use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, sysvar};
use anchor_spl::token::*;

use crate::state::transaction::Transaction;
use crate::state::user::User;
use crate::state::traits::Size;

#[derive(Accounts)]
pub struct CreateRequest<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Transaction::SIZE,
        seeds = [
            b"transaction",
            requester.to_account_info().key.as_ref(),
            requester.transactions.to_le_bytes().as_ref()],
        bump
    )]
    pub requester_transaction_account: Account<'info, Transaction>,

    #[account(
        init,
        payer = payer,
        space = 8 + Transaction::SIZE,
        seeds = [
            b"transaction",
            fulfiller.to_account_info().key.as_ref(),
            fulfiller.transactions.to_le_bytes().as_ref()],
        bump
    )]
    pub fulfiller_transaction_account: Account<'info, Transaction>,

    #[account(
        mut,
        seeds = [
            b"user",
            payer.key().as_ref(),
        ],
        bump
    )]
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
    pub sender_user_account: Box<Account<'info, User>>,

    #[account(
        init,
        payer = payer,
        space = 8 + Transaction::SIZE,
        seeds = [
            b"transaction",
            sender_user_account.to_account_info().key.as_ref(),
            sender_user_account.transactions.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub sender_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = sender_user_account, associated_token::mint = mint)]
    pub sender_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub receiver_user_account: Account<'info, User>,

    #[account(
        init,
        payer = payer,
        space = 8 + Transaction::SIZE,
        seeds = [
            b"transaction",
            receiver_user_account.to_account_info().key.as_ref(),
            receiver_user_account.transactions.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub receiver_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = receiver_user_account, associated_token::mint = mint)]
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FulfillRequest<'info> {
    #[account(
        seeds = [
            b"user",
            payer.key().as_ref(),
        ],
        bump
    )]
    pub fulfiller: Box<Account<'info, User>>,
    
    #[account(mut)]
    pub fulfiller_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = fulfiller, associated_token::mint = mint)]
    pub fulfiller_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub requester: Box<Account<'info, User>>,

    #[account(mut)]
    pub requester_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = requester, associated_token::mint = mint)]
    pub requester_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>
}

