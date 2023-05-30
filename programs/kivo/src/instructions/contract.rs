use anchor_lang::{prelude::*, solana_program::{system_program, sysvar}};
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, TokenAccount}};
use std::mem::size_of;
use clockwork_sdk::{state::{Thread, ThreadAccount}};
use crate::state::contract::*;
use crate::state::user::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct CreatePayment<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [
            b"payment",
            user_account.key().as_ref(),
            mint.key().as_ref(),
            receipient.key().as_ref(),
            user_account.num_contracts.to_le_bytes().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Payment>(),
    )]
    pub payment: Account<'info, Payment>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    /// CHECK: validated by payment account seeds
    pub receipient: UncheckedAccount<'info>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(mut)]
    pub user_account: Account<'info, User>,

    /// CHECK: validated by signer seeds
    pub sender: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, token::Token>,

    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct DisbursePayment<'info> {
    /// CHECK: authority validated by Payment account
    #[account(address = payment.authority)]
    pub authority: UncheckedAccount<'info>,
    #[account(mut, associated_token::authority = authority, associated_token::mint = mint)]
    pub authority_token_account: Account<'info, TokenAccount>,
    #[account(address = payment.mint)]
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"payment",
            payment.authority.as_ref(),
            payment.mint.as_ref(),
            payment.receipient.as_ref(),
        ],
        bump,
        has_one = authority,
        has_one = mint,
        has_one = receipient,
    )]
    pub payment: Box<Account<'info, Payment>>,
    #[account(
        signer,
        address = thread.pubkey(),
        constraint = thread.authority.eq(&payment.authority),
    )]
    pub thread: Box<Account<'info, Thread>>,
    /// CHECK:: receipient validated by Payment account
    #[account(address = payment.receipient)]
    pub receipient: UncheckedAccount<'info>,
    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, token::Token>,
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(amount: Option<u64>)]
pub struct UpdatePayment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"payment",
            payment.authority.key().as_ref(),
            payment.mint.key().as_ref(),
            payment.receipient.key().as_ref(),
        ],
        bump,
        has_one = authority,
    )]
    pub payment: Account<'info, Payment>,
}