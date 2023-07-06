use anchor_lang::prelude::*;

use crate::instructions::passive::*;

declare_id!("7aQcTJCAtyWLxEfysNdSBoshCFU1DyiFhkkzEkNmpSWL");

pub mod state;
pub mod instructions;

#[program]
pub mod kivo_yield_program {
    use super::*;

    pub fn handle_intialize_lending_account(ctx: Context<InitializeLendingAccount>) -> Result<()> {
        let kivo_account = &mut ctx.accounts.kivo_account;
        let marginfi_account = &mut ctx.accounts.marginfi_account;
        let lending_account = &mut ctx.accounts.lending_account;

        lending_account.new(
            kivo_account.key(),
            marginfi_account.key(),
        )?;

        Ok(())
    }
}   

