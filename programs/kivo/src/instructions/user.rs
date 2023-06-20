use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program};
use anchor_spl::token::*;
use anchor_spl::associated_token::*;

use crate::state::user::Username;
use crate::state::user::User;
use crate::state::user::Friend;
use crate::state::traits::Size;

pub const USER: &[u8] = b"user";
pub const USERNAME: &[u8] = b"username";
pub const FRIEND: &[u8] = b"friend";

#[derive(Accounts)]
#[instruction(name: [u8; 16])]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Username::SIZE,
        seeds = [
            USERNAME, 
            name.as_ref()
        ],
        bump
    )]
    pub username_account: Box<Account<'info, Username>>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<User>(),
        seeds = [
            USER,
            payer.key.as_ref()
        ], 
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

    // #[account(mut, associated_token::authority = depositor, associated_token::mint = mint)]
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,

    /// CHECK:
    pub user_account: UncheckedAccount<'info>,

    // #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    #[account(mut)]
    pub pda_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    // #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    /// CHECK: Validated by signer seeds
    pub withdrawer: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = withdrawer, associated_token::mint = mint)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub pda_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UnwrapWithdrawal<'info> {
    /// CHECK: Validated by signer seeds
    pub withdrawer: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = withdrawer, associated_token::mint = mint)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [
            b"unwrap",
            user_account.key().as_ref(),
            user_account.total_withdraws.to_le_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = user_account,
    )]
    pub temporary_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(new_name: [u8; 16])]
pub struct EditUsername<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Username::SIZE,
        seeds = [
            USERNAME, 
            new_name.as_ref()
        ],
        bump,
    )]
    pub new_username_account: Account<'info, Username>,

    #[account(
        mut,
        has_one = user_account,
    )]
    pub old_username_account: Account<'info, Username>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
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
        space = 8 + Friend::SIZE,
        seeds = [
            FRIEND,
            user_account.to_account_info().key.as_ref(),
            user_account.num_friends.to_le_bytes().as_ref()
        ],        
        bump
    )]
    pub new_friend: Account<'info, Friend>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    pub friend_account: Account<'info, User>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPreferredToken<'info> {
    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account()]
    pub preferred_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

