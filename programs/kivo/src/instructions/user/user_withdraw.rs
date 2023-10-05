use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::{token::*, associated_token::AssociatedToken};
use crate::state::user::User;

pub fn process(ctx: Context<Withdrawal>, amount: u64, withdraw_all: Option<bool>) -> Result<()> {
    msg!("Withdrawing");

    let bump = User::get_user_address(ctx.accounts.payer.key()).1;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];    

    if withdraw_all.is_some() {
        let wd_all = ctx.accounts.pda_token_account.amount;

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.pda_token_account.to_account_info(),
                    to: ctx.accounts.withdrawer_token_account.to_account_info(),
                    authority: ctx.accounts.user_account.to_account_info(),
                },
                signer_seeds
            ),
            wd_all
        )?;
    } else {
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.pda_token_account.to_account_info(),
                    to: ctx.accounts.withdrawer_token_account.to_account_info(),
                    authority: ctx.accounts.user_account.to_account_info(),
                },
                signer_seeds
            ),
            amount
        )?;
    }

    ctx.accounts.user_account.increment_withdrawals();

    ctx.accounts.user_account.exit(&crate::id())?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    /// CHECK: Validated by signer seeds
    pub withdrawer: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer, 
        associated_token::authority = withdrawer, 
        associated_token::mint = mint)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut, associated_token::authority = user_account, associated_token::mint = mint)]
    pub pda_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}