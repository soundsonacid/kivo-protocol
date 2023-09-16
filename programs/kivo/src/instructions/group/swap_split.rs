use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        program::invoke_signed,
    }
};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::*;

use crate::state::{
    group::Group,
    group::Balance,
    user::User
};
use crate::error::KivoError;
use crate::jupiter::Jupiter;


pub fn process(ctx: Context<SwapSplit>, amt: u64, output_amt_low_confidence: u64, data: Vec<u8>) -> Result<()> {
    require!(ctx.accounts.sender_input_balance.balance > amt, KivoError::BadWithdrawal);

    let bump = Group::get_group_address(ctx.accounts.group.admin.key(), ctx.accounts.group.identifier).1;
    let bump_bytes = bytemuck::bytes_of(&bump);
    let identifier_bytes = &ctx.accounts.group.identifier.to_le_bytes();

    let sig_seeds = Group::get_group_signer_seeds(&ctx.accounts.group.admin, identifier_bytes, bump_bytes);
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
        signer_seeds,
    )?;

    ctx.accounts.group_output_vault.reload()?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                to: ctx.accounts.receiver_vault.to_account_info(),
                from: ctx.accounts.group_output_vault.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            },
            signer_seeds
        ),
        (0.995 * output_amt_low_confidence as f64) as u64
    )?;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                to: ctx.accounts.kivo_vault.to_account_info(),
                from: ctx.accounts.group_output_vault.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            },
            signer_seeds
        ),
        (0.0045 * output_amt_low_confidence as f64) as u64
    )?;

    ctx.accounts.sender_input_balance.decrement_balance(amt);
    ctx.accounts.sender_input_balance.exit(&crate::id())?;
    Ok(())
}

#[derive(Accounts)]
pub struct SwapSplit<'info> {
    pub group: Box<Account<'info, Group>>,

    #[account(mut, associated_token::mint = input_mint, associated_token::authority = group)]
    pub group_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        associated_token::mint = output_mint,
        associated_token::authority = group,
        payer = payer,
    )]
    pub group_output_vault: Box<Account<'info, TokenAccount>>,

    #[account(address = User::get_user_address(payer.key()).0)]
    pub sender: Box<Account<'info, User>>,

    #[account(
        mut,
        seeds = [
            sender.key().as_ref(),
            group.key().as_ref(),
            input_mint.key().as_ref(),
        ],
        bump
    )]
    pub sender_input_balance: Box<Account<'info, Balance>>,

    pub receiver_vault: Box<Account<'info, TokenAccount>>,

    pub kivo_vault: Box<Account<'info, TokenAccount>>,

    pub input_mint: Box<Account<'info, Mint>>,

    pub output_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub jupiter_program: Program<'info, Jupiter>,

    pub system_program: Program<'info, System>,
}