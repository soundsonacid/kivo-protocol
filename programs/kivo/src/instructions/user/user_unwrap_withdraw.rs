use anchor_lang::{
    prelude::*,
    solana_program::{system_program, native_token::LAMPORTS_PER_SOL}
};
use anchor_spl::token::*;
use crate::{
    state::user::User,
    constants::UNWRAP,
};

pub fn process(ctx: Context<UnwrapWithdrawal>, amount: u64, withdraw_all: Option<bool>) -> Result<()> {
    msg!("Unwrapping & withdrawing");

    let bump = User::get_user_address(ctx.accounts.payer.key()).1;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];   

    if withdraw_all.is_some() {
        // Leave 0.05 SOL left over to pay other tx fees
        let wd_all = ctx.accounts.user_token_account.amount - (LAMPORTS_PER_SOL / 20);

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.temporary_token_account.to_account_info(),
                    authority: ctx.accounts.user_account.to_account_info(),
                },
                signer_seeds
            ),
            wd_all
        )?;
    } else {
        let actual_wd: u64;

        // Ensure that 0.05 SOL is left over to pay other tx fees
        if (ctx.accounts.user_token_account.amount - amount) < (LAMPORTS_PER_SOL / 20) {
            actual_wd = amount + (ctx.accounts.user_token_account.amount - amount);
        } else {
            actual_wd = amount;
        }

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.temporary_token_account.to_account_info(),
                    authority: ctx.accounts.user_account.to_account_info(),
                },
                signer_seeds
            ),
            actual_wd
        )?;
    }

    close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.temporary_token_account.to_account_info(),
                destination: ctx.accounts.withdrawer.to_account_info(),
                authority: ctx.accounts.user_account.to_account_info(),
            },
            signer_seeds
        )
    )?;

    ctx.accounts.user_account.increment_withdrawals();

    ctx.accounts.user_account.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct UnwrapWithdrawal<'info> {
    /// CHECK: Validated by signer seeds
    pub withdrawer: UncheckedAccount<'info>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [
            UNWRAP,
            user_account.key().as_ref(),
            user_account.total_withdraws.to_le_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = mint,
        token::authority = user_account,
    )]
    pub temporary_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}