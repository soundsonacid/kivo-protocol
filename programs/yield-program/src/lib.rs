use anchor_lang::prelude::*;
use instructions::*;

declare_id!("7aQcTJCAtyWLxEfysNdSBoshCFU1DyiFhkkzEkNmpSWL");

pub mod state;
mod instructions;

#[program]
pub mod kivo_yield_program {
    use super::*;

    pub fn handle_passive_lending_account_initialize(
            ctx: Context<InitializePassiveLendingAccount>, 
            bump: u8
        ) -> Result<()> {
        passive_initialize::process(ctx, bump)
    }

    pub fn handle_passive_lending_account_deposit(
            ctx: Context<PassiveLendingAccountDeposit>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
        passive_deposit::process(ctx, amount, bump)
    }

    pub fn handle_passive_lending_account_withdraw(
            ctx: Context<PassiveLendingAccountWithdraw>, 
            amount: u64, 
            bump: u8, 
            withdraw_all:  Option<bool>
        ) -> Result<()> {
        passive_withdraw::process(ctx, amount, bump, withdraw_all)
    }

    pub fn handle_passive_lending_account_borrow(
            ctx: Context<PassiveLendingAccountBorrow>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
        passive_borrow::process(ctx, amount, bump)
    }

    pub fn handle_passive_lending_account_repay(
            ctx: Context<PassiveLendingAccountRepay>, 
            amount: u64, 
            repay_all: Option<bool>, 
            bump: u8
        ) -> Result<()> {
        passive_repay::process(ctx, amount, bump, repay_all)
    }

    pub fn handle_passive_lending_account_claim_interest(
            ctx: Context<PassiveLendingAccountClaim>,
            amount: u64, 
            bump: u8
        ) -> Result<()> {
        passive_claim::process(ctx, amount, bump)
    }
}   

