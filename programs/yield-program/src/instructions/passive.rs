use anchor_lang::prelude::*;
use anchor_spl::token::*;
use marginfi::{
    program::Marginfi, 
    state::{
        marginfi_account::MarginfiAccount, 
        marginfi_group::{
            Bank, MarginfiGroup
        }
    }
};

use crate::state::lending_account::PassiveLendingAccount;

pub const LENDING_ACCOUNT: &[u8] = b"passive_lending_account";
pub const USER: &[u8] = b"user";
pub const KIVO_MFI_ACCOUNT: &[u8] = b"kivo_mfi_account";

#[derive(Accounts)]
pub struct InitializePassiveLendingAccount<'info> {
    /// CHECK: validated by address derivation
    #[account(address = kivo::state::user::User::get_user_address(payer.key()).0)]
    pub kivo_account: UncheckedAccount<'info>,

    /// CHECK: validated by mfi cpi call
    #[account(
        mut,
        seeds = [
            KIVO_MFI_ACCOUNT,
            kivo_account.key().as_ref(),
        ],
        bump,
    )]
    pub marginfi_account: UncheckedAccount<'info>,

    /// CHECK: validated by mfi cpi call
    pub marginfi_group: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = std::mem::size_of::<PassiveLendingAccount>(),
        seeds = [
            LENDING_ACCOUNT,
            kivo_account.key().as_ref()
        ],
        bump,
    )]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated by mfi cpi
    pub marginfi_program: Program<'info, Marginfi>,

    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassiveLendingAccountDeposit<'info> {
    /// CHECK: validated by address derivation
    #[account(address = kivo::state::user::User::get_user_address(payer.key()).0)]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            KIVO_MFI_ACCOUNT,
            kivo_account.key().as_ref(),
        ],
        bump,
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,

    /// CHECK: validated by mfi cpi call
    pub marginfi_group: UncheckedAccount<'info>,

    pub marginfi_bank: AccountLoader<'info, Bank>,

    /// CHECK: validated by mfi cpi call -> banks tied to specific mfi group -> bad vault fails cpi
    pub bank_vault: UncheckedAccount<'info>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(payer.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account(address = marginfi_bank.load()?.mint)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated by mfi cpi
    pub marginfi_program: Program<'info, Marginfi>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassiveLendingAccountWithdraw<'info> {
    /// CHECK: validated by address derivation
    #[account(address = kivo::state::user::User::get_user_address(payer.key()).0)]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            KIVO_MFI_ACCOUNT,
            kivo_account.key().as_ref(),
        ],
        bump,
    )]
    pub marginfi_account: AccountLoader<'info, MarginfiAccount>,

    /// CHECK: validated by mfi cpi
    pub marginfi_group: UncheckedAccount<'info>,

    pub marginfi_bank: AccountLoader<'info, Bank>,

    /// CHECK: validated by mfi cpi
    #[account(mut)]
    pub bank_vault: UncheckedAccount<'info>,

    /// CHECK: validated by mfi cpi
    #[account(mut)]
    pub bank_vault_authority: UncheckedAccount<'info>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(payer.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account(address = marginfi_bank.load()?.mint)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated by mfi cpi
    pub marginfi_program: Program<'info, Marginfi>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassiveLendingAccountClaimInterest<'info> {
    /// CHECK: validated by address derivation
    #[account(address = kivo::state::user::User::get_user_address(payer.key()).0)]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(payer.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: validated by mfi cpi
    pub marginfi_program: Program<'info, Marginfi>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: Program<'info, System>,
}