use anchor_lang::prelude::*;

use crate::instructions::passive::*;

declare_id!("7aQcTJCAtyWLxEfysNdSBoshCFU1DyiFhkkzEkNmpSWL");

pub mod state;
pub mod instructions;

#[program]
pub mod kivo_yield_program {
    use super::*;

    pub fn handle_initialize_passive_lending_account(ctx: Context<InitializePassiveLendingAccount>, bump: u8) -> Result<()> {
        let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];  

        let init_margin_acc = marginfi::cpi::accounts::MarginfiAccountInitialize {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            authority: ctx.accounts.kivo_account.to_account_info(),
            fee_payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let init_margin_acc_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            init_margin_acc,
            signer_seeds
        );

        marginfi::cpi::marginfi_account_initialize(init_margin_acc_ctx)?;

        ctx.accounts.passive_lending_account.new(
            ctx.accounts.kivo_account.key(),
            ctx.accounts.marginfi_account.key(),
            ctx.accounts.marginfi_group.key(),
        )?;

        Ok(())
    }

    pub fn handle_passive_lending_account_deposit(ctx: Context<PassiveLendingAccountDeposit>, amount: u64, bump: u8) -> Result<()> {
        let passive_lending_account = &mut ctx.accounts.passive_lending_account;

        passive_lending_account.increment_deposits(amount);
        Ok(())
    }

    pub fn handle_passive_lending_account_withdraw(ctx: Context<PassiveLendingAccountWithdraw>, amount: u64, bump: u8) -> Result<()> {
        let passive_lending_account = &mut ctx.accounts.passive_lending_account;

        passive_lending_account.increment_withdrawals(amount);
        Ok(())
    }

    pub fn handle_passive_lending_account_claim_interest(ctx: Context<PassiveLendingAccountClaimInterest>, amount: u64, bump: u8) -> Result<()> {

        Ok(())
    }
}   

