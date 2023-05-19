use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::state::transaction::Transaction;
use crate::state::user::User;

#[derive(Accounts)]
pub struct CreateTransactionAccount<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 8 + 8 + 32 + 1, // pk + pk + u64 + u64 + pk + bool
        seeds = [b"transaction",
                 user_account.to_account_info().key.as_ref(),
                 user_account.payments_sent.to_le_bytes().as_ref()],
        bump
    )]
    pub user_transaction_account: Account<'info, Transaction>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub receiver_account: Account<'info, User>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + 8 + 8 + 32 + 1, // pk + u16 + u64 + u64 + pk + bool
        seeds = [b"transaction",
                receiver_account.to_account_info().key.as_ref(),
                receiver_account.payments_received.to_le_bytes().as_ref()],
        bump
    )]
    pub receiver_transaction_account: Account<'info, Transaction>,
    pub token: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sender: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sender_user_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExecuteSwapTransaction<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sender: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sender_user_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub sender_source_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sender_destination_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub swap_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub swap_authority: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub swap_source: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub swap_destination: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pool_mint: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pool_fee: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_swap_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub jupiter_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ExecuteSwapTransaction<'info> {
    pub fn get_swap_cpi_context<'a>(&self, signer_seeds: &'a [&'a [&'a [u8]]]) 
        -> CpiContext<'a, 'a, 'a, 'info, jupiter_cpi::cpi::accounts::TokenSwap<'info>> {

        let accounts = jupiter_cpi::cpi::accounts::TokenSwap {
            token_swap_program: self.token_swap_program.to_account_info(),
            token_program: self.token_program.to_account_info().clone(),
            swap: self.swap_account.to_account_info(),
            authority: self.swap_authority.to_account_info(),
            user_transfer_authority: self.sender_user_account.to_account_info(),
            source: self.sender_source_token_account.to_account_info(),
            swap_source: self.swap_source.to_account_info(),
            swap_destination: self.swap_destination.to_account_info(),
            destination: self.sender_destination_token_account.to_account_info(),
            pool_mint: self.pool_mint.to_account_info(),
            pool_fee: self.pool_fee.to_account_info(),
        };

        CpiContext::new_with_signer(self.jupiter_program.to_account_info(), accounts, signer_seeds)
    }
}