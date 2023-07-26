// Created by Frank
use anchor_lang::prelude::*;
use instructions::*;
pub mod state;
pub mod constants;
mod instructions;

declare_id!("FUcr3cVqbuQb7fHnCVzeLMcwPKQ3joH3kfxEwMXd96is");

#[program]
pub mod kivo_yield_program {
    use super::*;

    // Passive Lending endpoints
    // 1. handle_passive_lending_account_initialize
    // 2. handle_initialize_lending_vaults
    // 3. handle_passive_lending_account_deposit
    // 4. handle_passive_lending_account_withdraw
    // 5. handle_passive_lending_account_borrow
    // 6. handle_passive_lending_account_repay

    pub fn handle_passive_lending_account_initialize(
            ctx: Context<PassiveLendingAccountInitialize>, 
        ) -> Result<()> {
        passive_initialize::process(ctx)
    }

    pub fn handle_initialize_lending_vaults(
        ctx: Context<InitializeLendingVaults>,
    ) -> Result<()> {
        passive_init_vaults::process(ctx)
    }

    pub fn handle_passive_lending_account_deposit(
            ctx: Context<PassiveLendingAccountDeposit>, 
            amount: u64, 
        ) -> Result<()> {
        passive_deposit::process(ctx, amount)
    }

    pub fn handle_passive_lending_account_withdraw(
            ctx: Context<PassiveLendingAccountWithdraw>, 
            amount: u64, 
            withdraw_all:  Option<bool>
        ) -> Result<()> {
        passive_withdraw::process(ctx, amount, withdraw_all)
    }

    pub fn handle_passive_lending_account_borrow(
            ctx: Context<PassiveLendingAccountBorrow>, 
            amount: u64, 
        ) -> Result<()> {
        passive_borrow::process(ctx, amount)
    }

    pub fn handle_passive_lending_account_repay(
            ctx: Context<PassiveLendingAccountRepay>, 
            amount: u64, 
            repay_all: Option<bool>, 
        ) -> Result<()> {
        passive_repay::process(ctx, amount, repay_all)
    }

    pub fn handle_passive_lending_account_claim_interest(
            ctx: Context<PassiveLendingAccountClaim>,
            amount: u64, 
        ) -> Result<()> {
        passive_claim::process(ctx, amount)
    }
}