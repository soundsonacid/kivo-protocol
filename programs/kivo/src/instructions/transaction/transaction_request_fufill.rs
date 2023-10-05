use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use anchor_spl::token::*;
use crate::state::{
        user::User,
        transaction::Transaction,
};
use crate::error::KivoError;

pub fn process(ctx: Context<FulfillRequest>, amount: u64) -> Result<()> {
    msg!("Fulfilling transaction!");

    let fulfiller = &ctx.accounts.fulfiller;
    let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
    let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;

    let bump = User::get_user_address(ctx.accounts.payer.key()).1;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];

    let fee = amount / 400;

    let amt_final = amount - fee;

    require!(amt_final > 0, KivoError::NegDelta);

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.fulfiller_token_account.to_account_info(),
                to: ctx.accounts.kivo_vault.to_account_info(),
                authority: fulfiller.to_account_info(),
            },
            signer_seeds
        ),
        fee
    )?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.fulfiller_token_account.to_account_info(),
                to: ctx.accounts.requester_token_account.to_account_info(),
                authority: fulfiller.to_account_info(),
            },
            signer_seeds
        ),
        amt_final
    )?;

    fulfiller_transaction_account.fulfill(amt_final);
    requester_transaction_account.fulfill(amt_final);

    fulfiller_transaction_account.exit(&crate::id())?;
    requester_transaction_account.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct FulfillRequest<'info> {
    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub fulfiller: Box<Account<'info, User>>,
    
    #[account(mut)]
    pub fulfiller_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = fulfiller, associated_token::mint = mint)]
    pub fulfiller_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub requester: Box<Account<'info, User>>,

    #[account(mut)]
    pub requester_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = requester, associated_token::mint = mint)]
    pub requester_token_account: Box<Account<'info, TokenAccount>>,

    pub kivo_vault: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>
}