use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program};
use anchor_spl::token::*;
use anchor_spl::associated_token::*;
use std::mem::size_of;

use crate::state::user::Username;
use crate::state::user::User;
use crate::state::user::Friend;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Username>(),
        seeds = [b"username", name.as_bytes()],
        bump
    )]
    pub username_account: Box<Account<'info, Username>>,

    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<User>(),
        seeds = [b"user", payer.key.as_ref()], 
        bump,
    )]
    pub user_account: Box<Account<'info, User>>,  

    #[account()]
    pub wsol_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = wsol_mint,
        associated_token::authority = user_account,
    )]
    pub wsol_vault: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub usdc_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = usdc_mint,
        associated_token::authority = user_account,
    )]
    pub usdc_vault: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub usdt_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = usdt_mint,
        associated_token::authority = user_account,
    )]
    pub usdt_vault: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub dai_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = dai_mint,
        associated_token::authority = user_account,
    )]
    pub dai_vault: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub bonk_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = bonk_mint,
        associated_token::authority = user_account,
    )]
    pub bonk_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,                

    /// CHECK: 
    pub owner: UncheckedAccount<'info>,      

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: validated by cpi context
    pub depositor: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = depositor, associated_token::mint = mint)]
    pub depositor_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"user",
            payer.key().as_ref()
        ],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub pda_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    /// CHECK: Validated by signer seeds
    pub withdrawer: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = withdrawer, associated_token::mint = mint)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"user",
            payer.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub pda_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(new_name: String)]
pub struct EditUsername<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Username>(),
        seeds = [b"username", new_name.as_bytes()],
        bump,
        has_one = user_account,
    )]
    pub new_username_account: Account<'info, Username>,

    #[account(
        mut,
        has_one = user_account,
    )]
    pub old_username_account: Account<'info, Username>,

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
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddFriend<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Friend>(),
        seeds = [
            b"friend",
            user_account.to_account_info().key.as_ref(),
            user_account.num_friends.to_le_bytes().as_ref()
        ],        
        bump
    )]
    pub new_friend: Account<'info, Friend>,

    #[account(
        mut,
        seeds = [
            b"user",
            payer.key().as_ref(),
        ],
        bump
    )]
    pub user_account: Account<'info, User>,

    pub friend_account: Account<'info, User>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}