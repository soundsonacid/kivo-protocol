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
        let kivo_signer_seeds = &[&signature_seeds[..]];  

        let init_mfi_acc = marginfi::cpi::accounts::MarginfiAccountInitialize {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            authority: ctx.accounts.kivo_account.to_account_info(),
            fee_payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let init_mfi_acc_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            init_mfi_acc,
            kivo_signer_seeds
        );

        marginfi::cpi::marginfi_account_initialize(init_mfi_acc_ctx)?;

        ctx.accounts.passive_lending_account.new(
            ctx.accounts.kivo_account.key(),
            ctx.accounts.marginfi_account.key(),
            ctx.accounts.marginfi_group.key(),
        )?;

        Ok(())
    }

    pub fn handle_passive_lending_account_deposit(ctx: Context<PassiveLendingAccountDeposit>, amount: u64, bump: u8) -> Result<()> {
        let passive_lending_account = &mut ctx.accounts.passive_lending_account;

        let signature_seeds = kivo::state::user::User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let kivo_signer_seeds = &[&signature_seeds[..]];  

        let mfi_deposit_acc = marginfi::cpi::accounts::LendingAccountDeposit {
            marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
            marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
            signer: ctx.accounts.kivo_account.to_account_info(),
            bank: ctx.accounts.marginfi_bank.to_account_info(),
            signer_token_account: ctx.accounts.kivo_token_account.to_account_info(),
            bank_liquidity_vault: ctx.accounts.bank_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let mfi_deposit_ctx = CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info().clone(),
            mfi_deposit_acc,
            kivo_signer_seeds
        );

        marginfi::cpi::lending_account_deposit(mfi_deposit_ctx, amount)?;

        passive_lending_account.increment_deposits(amount);
        passive_lending_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_passive_lending_account_withdraw(ctx: Context<PassiveLendingAccountWithdraw>, amount: u64, bump: u8) -> Result<()> {
        let passive_lending_account = &mut ctx.accounts.passive_lending_account;

        passive_lending_account.increment_withdrawals(amount);
        passive_lending_account.exit(&crate::id())?;
        Ok(())
    }

    pub fn handle_passive_lending_account_claim_interest(ctx: Context<PassiveLendingAccountClaimInterest>, amount: u64, bump: u8) -> Result<()> {

        Ok(())
    }
}   

