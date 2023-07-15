use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7aQcTJCAtyWLxEfysNdSBoshCFU1DyiFhkkzEkNmpSWL");

pub mod state;
mod instructions;

#[program]
pub mod kivo_yield_program {
    use super::*;

    pub fn handle_initialize_passive_lending_account(ctx: Context<InitializePassiveLendingAccount>, bump: u8) -> Result<()> {
        passive_initialize::handler(ctx, bump)
    }

    pub fn handle_passive_lending_account_deposit(ctx: Context<PassiveLendingAccountDeposit>, amount: u64, bump: u8) -> Result<()> {
        passive_deposit::handler(ctx, amount, bump)
    }

    pub fn handle_passive_lending_account_withdraw(ctx: Context<PassiveLendingAccountWithdraw>, amount: u64, bump: u8, withdraw_all:  Option<bool>) -> Result<()> {
        passive_withdraw::handler(ctx, amount, bump, withdraw_all)
    }

    pub fn handle_passive_lending_account_borrow(ctx: Context<PassiveLendingAccountBorrow>, amount: u64, bump: u8) -> Result<()> {
        passive_borrow::handler(ctx, amount, bump)
    }

    pub fn handle_passive_lending_account_repay(ctx: Context<PassiveLendingAccountRepay>, amount: u64, repay_all: Option<bool>, bump: u8) -> Result<()> {
        passive_repay::handler(ctx, amount, bump, repay_all)
    }

    pub fn handle_passive_lending_account_claim_interest(ctx: Context<PassiveLendingAccountClaim>, amount: u64, bump: u8) -> Result<()> {
        passive_claim::handler(ctx, amount, bump)
    }
}   

