use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::*;
use crate::constants::EMPTY_THRESHOLD;
use crate::state::{
    group::Balance,
    user::User
};
use crate::error::KivoError;

pub fn process(ctx: Context<WithdrawFromGroupWallet>, wd: u64, withdraw_all: Option<bool>) -> Result<()> {
    
    if ctx.accounts.user_balance.balance < wd {
        msg!("Overdraw of token {}", ctx.accounts.mint.key().to_string());
        msg!("User balance: {}", ctx.accounts.user_balance.balance);
        msg!("Attemped withdrawal: {}", wd);
        return Err(error!(KivoError::GroupWithdrawalExceedsBalance));
    }

    if withdraw_all.is_some() {
        let wd_all = ctx.accounts.user_balance.balance;

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.group_vault.to_account_info(),
                    to: ctx.accounts.user_vault.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                },
            ),
            wd_all
        )?;

        msg!("Withdrew {} of mint {}", wd_all, ctx.accounts.mint.key().to_string());
        msg!("Group: {}", ctx.accounts.group.key().to_string());
        msg!("Group vault: {}", ctx.accounts.group_vault.key().to_string());
        msg!("Destination vault: {}", ctx.accounts.user_vault.key().to_string());

        // Adjust user balance accordingly 
        ctx.accounts.user_balance.decrement_balance(wd_all);
        ctx.accounts.user_balance.reload()?;

        if ctx.accounts.user_balance.balance.gt(&EMPTY_THRESHOLD) {
            msg!("withdraw_all left {} of {} remaining in Balance.", 
            ctx.accounts.user_balance.balance,
            ctx.accounts.mint.key().to_string(),
        );
        }

        ctx.accounts.user_balance.exit(&crate::id())?;
    } else {
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.group_vault.to_account_info(),
                    to: ctx.accounts.user_vault.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                },
            ),
            wd
        )?;

        msg!("Withdrew {} of mint {}", wd, ctx.accounts.mint.key().to_string());
        msg!("Group: {}", ctx.accounts.group.key().to_string());
        msg!("Group vault: {}", ctx.accounts.group_vault.key().to_string());
        msg!("Destination vault: {}", ctx.accounts.user_vault.key().to_string());

        // Adjust user balance accordingly 
        ctx.accounts.user_balance.decrement_balance(wd);
        ctx.accounts.user_balance.exit(&crate::id())?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFromGroupWallet<'info> {
    #[account(mut, associated_token::mint = mint, associated_token::authority = group)]
    pub group_vault: Account<'info, TokenAccount>,

    pub user: Account<'info, User>,

    #[account(
         init_if_needed,
         associated_token::mint = mint, 
         associated_token::authority = user,
         payer = payer,
    )]
    pub user_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            user.key().as_ref(),
            group.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub user_balance: Account<'info, Balance>,

    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub group: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}