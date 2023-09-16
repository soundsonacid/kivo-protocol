use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        program::invoke_signed,
    }
};
use anchor_spl::token::*;
use crate::state::{
    user::User,
    transaction::Transaction,
};
use crate::jupiter::Jupiter;

pub fn process(ctx: Context<FulfillSwap>, output_amt_low_confidence: u64, data: Vec<u8>) -> Result<()> {

    let bump = User::get_user_address(ctx.accounts.payer.key()).1;
    let sig_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&sig_seeds[..]];

    let accounts: Vec<AccountMeta> = ctx.remaining_accounts
        .iter()
        .map(|acc| AccountMeta {
            pubkey: *acc.key,
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        })
        .collect();

    let account_infos: Vec<AccountInfo> = ctx.remaining_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    invoke_signed(
        &Instruction {
            program_id: *ctx.accounts.jupiter_program.key,
            accounts,
            data,
        },
        &account_infos,
        signer_seeds
    )?;

    ctx.accounts.fulfiller_output_token_account.reload()?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.fulfiller_output_token_account.to_account_info(),
                to: ctx.accounts.requester_token_account.to_account_info(),
                authority: ctx.accounts.fulfiller.to_account_info(),
            },
            signer_seeds
        ),
        (0.995 * output_amt_low_confidence as f64) as u64
    )?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.fulfiller_output_token_account.to_account_info(),
                to: ctx.accounts.requester_token_account.to_account_info(),
                authority: ctx.accounts.fulfiller.to_account_info(),
            },
            signer_seeds
        ),
        (0.004 * output_amt_low_confidence as f64) as u64
    )?;

    ctx.accounts.fulfiller_transaction_account.fulfill();
    ctx.accounts.requester_transaction_account.fulfill();

    ctx.accounts.fulfiller_transaction_account.exit(&crate::id())?;
    ctx.accounts.requester_transaction_account.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct FulfillSwap<'info> {
    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub fulfiller: Box<Account<'info, User>>,

    #[account(mut)]
    pub requester: Box<Account<'info, User>>,

    #[account(mut)]
    pub fulfiller_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut)]
    pub requester_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = fulfiller, associated_token::mint = input_mint)]
    pub fulfiller_input_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::authority = fulfiller, associated_token::mint = output_mint)]
    pub fulfiller_output_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::authority = requester, associated_token::mint = output_mint)]
    pub requester_token_account: Box<Account<'info, TokenAccount>>,

    pub input_mint: Box<Account<'info, Mint>>,

    pub output_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub kivo_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub jupiter_program: Program<'info, Jupiter>,

    pub token_program: Program<'info, Token>,
}