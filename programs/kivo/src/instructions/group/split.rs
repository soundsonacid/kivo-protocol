use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::state::{
    group::Balance,
    user::User
};
use crate::constants::ZERO;
use crate::error::KivoError;

pub fn process(ctx: Context<Split>, amt: u64) -> Result<()> {

    if ctx.accounts.user_balance.balance.lt(&amt) {
        msg!("Overuse of token {}", ctx.accounts.mint.key().to_string());
        msg!("User balance: {}", ctx.accounts.user_balance.balance);
        msg!("Attemped usage: {}", amt);
        return Err(error!(KivoError::ModeUsageExceedsBalance));
    }

    // Figure out how much we have in the group vault to begin with
    let bal_pre = ctx.accounts.group_vault.amount;
    msg!("Initial balance: {}", bal_pre);

    // Transfer amt of mint to the receiver's destination vault
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.group_vault.to_account_info(),
                to: ctx.accounts.destination_vault.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            },
        ),
        amt
    )?;

    // Figure out how much we now have in the group vault
    ctx.accounts.group_vault.reload()?;
    let bal_post = ctx.accounts.group_vault.amount;
    msg!("Final balance: {}", bal_post);

    // Figure out how much precisely left the group vault
    // We could just use amt for this probably
    // I'll play around with it - 10/4/23
    // I've decided we will keep for error checking purposes - 10/5/23
    let bal_delta = bal_pre - bal_post;
    msg!("Balance delta: {}", bal_delta);

    if bal_delta.le(&ZERO) {
        msg!("Balance change for token {} in vault {} is LTE zero", 
            ctx.accounts.mint.key().to_string(), 
            ctx.accounts.group_vault.key().to_string()
        );
        msg!("Initial balance: {}", bal_pre);
        msg!("Final balance: {}", bal_post);
        msg!("Balance delta: {}", bal_delta);
        return Err(error!(KivoError::NegDelta));
    }

    // Adjust user balance accordingly
    ctx.accounts.user_balance.decrement_balance(bal_delta);
    msg!("Balance {} for mint {} and group {} owned by {} decreased by {}", 
        ctx.accounts.user_balance.key().to_string(), 
        ctx.accounts.mint.key().to_string(),
        ctx.accounts.group.key().to_string(),
        ctx.accounts.sender.key().to_string(),
        bal_delta
    );

    ctx.accounts.user_balance.exit(&crate::id())?;
    ctx.accounts.user_balance.reload()?;

    msg!("New balance: {}", ctx.accounts.user_balance.balance);

    msg!("{} of mint {} sent to {}",
        amt,
        ctx.accounts.mint.key().to_string(),
        ctx.accounts.receiver.key.to_string(),
    );

    Ok(())
}

#[derive(Accounts)]
pub struct Split<'info> {
    #[account(mut, associated_token::mint = mint, associated_token::authority = group)]
    pub group_vault: Account<'info, TokenAccount>,

    #[account(address = User::get_user_address(payer.key()).0)]
    pub sender: Account<'info, User>,

    /// CHECK:
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = receiver)]
    pub destination_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            sender.key().as_ref(),
            group.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub user_balance: Account<'info, Balance>,

    pub mint: Account<'info, Mint>,
    
    pub group: Signer<'info>,

    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}