use anchor_lang::prelude::*;

use crate::instructions::passive::*;

declare_id!("7aQcTJCAtyWLxEfysNdSBoshCFU1DyiFhkkzEkNmpSWL");

pub mod state;
pub mod instructions;

#[program]
pub mod kivo_yield_program {
    use super::*;

    pub fn handle_intialize_lending_account(ctx: Context<InitializeLendingAccount>, bump: u8) -> Result<()> {
        let kivo_account = &mut ctx.accounts.kivo_account;
        let marginfi_account = &mut ctx.accounts.marginfi_account;
        let marginfi_group = &ctx.accounts.marginfi_group;
        let lending_account = &mut ctx.accounts.lending_account;
        let system_program = &ctx.accounts.system_program;
        let marginfi_program: &ctx.accounts.marginfi_program;


        let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];  

        let init_margin_acc = MarginfiAccountInitialize {
            marginfi_group: marginfi_group,
            marginfi_account: marginfi_account,
            authority: kivo_account,
            fee_payer: kivo_account,
            system_program: system_program,
        }

        let init_margin_acc_ctx = CpiContext::new_with_signer(
            marginfi_program.to_account_info().clone(),
            init_margin_acc,
            marginfi_program
        );

        marginfi_account_initialize(init_margin_acc_ctx, signer_seeds)?;
        
        lending_account.new(
            kivo_account.key(),
            marginfi_account.key(),
        )?;

        Ok(())
    }
}   

