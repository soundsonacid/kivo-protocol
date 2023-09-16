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


// Should work for both entering & exiting an ape mode
// Executes a standard swap on jupiter between two arbitrary tokens
pub fn process(ctx: Context<Ape>, amt: u64, output_amt_low_confidence: u64, data: Vec<u8>) -> Result<()> {

    require!(ctx.accounts.user_input_balance.balance > amt, KivoError::BadWithdrawal);

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

    if !ctx.accounts.user_output_balance.initialized {
        ctx.accounts.user_output_balance.new(
            ctx.accounts.user.key(),
            ctx.accounts.group.key(),
            ctx.accounts.output_mint.key()
        )?;
    };

    ctx.accounts.user_input_balance.decrement_balance(amt);
    // This won't work right yet - need to use output amount instead of input.
    ctx.accounts.user_output_balance.increment_balance(output_amt_low_confidence);

    ctx.accounts.user_input_balance.exit(&crate::id())?;
    ctx.accounts.user_output_balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct Ape<'info> {
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
    pub user: Box<Account<'info, User>>,

    #[account(
        mut,
        seeds = [
            user.key().as_ref(),
            group.key().as_ref(),
            input_mint.key().as_ref(),
        ],
        bump
    )]
    pub user_input_balance: Box<Account<'info, Balance>>,

    #[account(
        init_if_needed,
        seeds = [
            user.key().as_ref(),
            group.key().as_ref(),
            output_mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = std::mem::size_of::<Balance>() + 8,
    )]
    pub user_output_balance: Box<Account<'info, Balance>>,

    pub input_mint: Box<Account<'info, Mint>>,

    pub output_mint: Box<Account<'info, Mint>>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub jupiter_program: Program<'info, Jupiter>,

    pub system_program: Program<'info, System>,
}