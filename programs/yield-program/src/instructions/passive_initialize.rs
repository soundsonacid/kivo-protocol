use anchor_lang::prelude::*;
use marginfi::program::Marginfi;
use crate::{
    state::lending_account::PassiveLendingAccount,
    constants::{
        KIVO_MFI_ACCOUNT, LENDING_ACCOUNT
    },
};

pub fn process(ctx: Context<PassiveLendingAccountInitialize>, bump: u8) -> Result<()> {
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

#[derive(Accounts)]
pub struct PassiveLendingAccountInitialize<'info> {
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