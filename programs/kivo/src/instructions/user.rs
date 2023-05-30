use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::*;
use std::mem::size_of;

use crate::state::user::Username;
use crate::state::user::User;
use crate::state::user::Friend;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeUser<'info> {
    // User Accounts
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
    pub user_account: Box<Account<'info, User>>,  // This should be a PDA
    // User Associated Token Accounts
    pub wsol_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = wsol_mint,
        associated_token::authority = user_account,
    )]
    pub wsol_vault: Box<Account<'info, TokenAccount>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = usdc_mint,
        associated_token::authority = user_account,
    )]
    pub usdc_vault: Box<Account<'info, TokenAccount>>,
    pub usdt_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = usdt_mint,
        associated_token::authority = user_account,
    )]
    pub usdt_vault: Box<Account<'info, TokenAccount>>,
    pub dai_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = dai_mint,
        associated_token::authority = user_account,
    )]
    pub dai_vault: Box<Account<'info, TokenAccount>>,
    pub bonk_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = bonk_mint,
        associated_token::authority = user_account,
    )]
    pub bonk_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub payer: Signer<'info>,                // This should also be the public key of the client
    /// CHECK: 
    pub owner: UncheckedAccount<'info>,      // This should be the public key of the client 
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub depositor: UncheckedAccount<'info>,
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub pda_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub withdrawer: UncheckedAccount<'info>,
    #[account(mut)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,
     /// CHECK: This is not dangerous because we don't read or write from this account
    pub user_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub pda_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(new_name: String)]
pub struct EditUsername<'info> {
    #[account(mut)]
    pub old_username_account: Account<'info, Username>,
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 20,
        seeds = [b"username", new_name.as_bytes()],
        bump
    )]
    pub new_username_account: Account<'info, Username>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddFriend<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 4,
        seeds = [b"friend",
                 user_account.to_account_info().key.as_ref(),
                 user_account.num_friends.to_le_bytes().as_ref()],        
        bump
    )]
    pub new_friend: Account<'info, Friend>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    pub friend_account: Account<'info, User>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}