use anchor_lang::prelude::*;
use anchor_spl::token::*;
use marginfi::{
    program::Marginfi,
    state::{
        marginfi_group::Bank,
        marginfi_account::MarginfiAccount,
    }
};
use crate::{
    state::lending_account::PassiveLendingAccount,
    constants::KIVO_MFI_ACCOUNT,
};

pub fn process(ctx: Context<PassiveLendingAccountWithdraw>, amount: u64, withdraw_all:  Option<bool>) -> Result<()> {
    msg!("Withdrawing from mfi account");

    let withdraw_all = withdraw_all;

    let amount = if withdraw_all == Some(true) {
        0
    } else {
        amount
    };

    let lender_bump = PassiveLendingAccount::get_lender_address(ctx.accounts.kivo_account.key()).1;

    marginfi::cpi::lending_account_withdraw(
        CpiContext::new_with_signer(
            ctx.accounts.marginfi_program.to_account_info(),
            marginfi::cpi::accounts::LendingAccountWithdraw {
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                signer: ctx.accounts.passive_lending_account.to_account_info(),
                bank: ctx.accounts.marginfi_bank.to_account_info(),
                destination_token_account: ctx.accounts.kivo_token_account.to_account_info(),
                bank_liquidity_vault: ctx.accounts.bank_vault.to_account_info(),
                bank_liquidity_vault_authority: ctx.accounts.bank_vault_authority.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[&[
                KIVO_MFI_ACCOUNT.as_bytes(),
                ctx.accounts.kivo_account.key().as_ref(),
                &[lender_bump],
            ]],
        ),
        amount, 
        withdraw_all,
    )?;

    ctx.accounts.passive_lending_account.increment_withdrawals(amount);
    ctx.accounts.passive_lending_account.exit(&crate::id())?;

    msg!("Withdrew from mfi account");
    
    Ok(())
}

#[derive(Accounts)]
pub struct PassiveLendingAccountWithdraw<'info> {
    /// CHECK: validated by address derivation
    #[account(address = kivo::state::user::User::get_user_address(payer.key()).0)]
    pub kivo_account: UncheckedAccount<'info>,

    #[account(mut, associated_token::authority = kivo_account, associated_token::mint = mint)]
    pub kivo_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = PassiveLendingAccount::get_lender_address(kivo_account.key()).0)]
    pub passive_lending_account: Account<'info, PassiveLendingAccount>,

    #[account(
        mut,
        seeds = [
            KIVO_MFI_ACCOUNT.as_bytes(),
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