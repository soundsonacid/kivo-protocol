use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::{token::*, associated_token::AssociatedToken};
use crate::{
    state::{user::User, group::Balance},
    constants::UNWRAP, error::KivoError,
};

pub fn process(
    ctx: Context<WithdrawToWallet>,
    wd: u64,
    withdraw_all: Option<bool>,
) -> Result<()> {
    // Check if the mint is SOL
    let is_sol = ctx.accounts.mint.key() == spl_token::native_mint::id();

    let actual_wd = if let Some(_) = withdraw_all {
        ctx.accounts.user_balance.balance
    } else {
        wd
    };

    if ctx.accounts.user_balance.balance < actual_wd {
        msg!("Overdraw of token {}", ctx.accounts.mint.key().to_string());
        return Err(error!(KivoError::GroupWithdrawalExceedsBalance));
    }

    if is_sol {
        // Unwrap SOL
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.group_vault.to_account_info(),
                    to: ctx.accounts.temporary_token_account.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                }
            ),
            actual_wd
        )?;

        close_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount {
                    account: ctx.accounts.temporary_token_account.to_account_info(),
                    destination: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                }
            )
        )?;
    } else {
        // Standard token withdrawal
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.group_vault.to_account_info(),
                    to: ctx.accounts.user_vault.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                }
            ),
            actual_wd
        )?;

        close_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount {
                    account: ctx.accounts.temporary_token_account.to_account_info(),
                    destination: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                }
            )
        )?;

        // Decrement user balance by the actual withdrawal amount
        ctx.accounts.user_balance.decrement_balance(actual_wd);
    }

    // Common logic - exit the user balance
    ctx.accounts.user_balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawToWallet<'info> {
    pub withdrawer: Box<Account<'info, User>>,

    #[account(
        init_if_needed,
        associated_token::mint = mint, 
        associated_token::authority = payer,
        payer = payer,
   )]
    pub user_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub group_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            withdrawer.key().as_ref(),
            group.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub user_balance: Box<Account<'info, Balance>>,

    #[account(
        init,
        seeds = [
            UNWRAP,
            withdrawer.key().as_ref(),
            group.key().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = group,
    )]
    pub temporary_token_account: Box<Account<'info, TokenAccount>>,

    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub group: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}