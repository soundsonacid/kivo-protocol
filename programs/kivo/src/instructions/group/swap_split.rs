use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        program::invoke,
    }
};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::*;

use crate::state::{
    group::Balance,
    user::User
};
use crate::error::KivoError;
use crate::jupiter::Jupiter;


pub fn process(ctx: Context<SwapSplit>, amt: u64, data: Vec<u8>) -> Result<()> {
    require!(ctx.accounts.input_balance.balance > amt, KivoError::BadWithdrawal);

    // Get the amount of whatever output token group has before the swap takes place
    let bal_pre = ctx.accounts.group_output_vault.amount;
        
    // Compile remaining accounts into ais for instruction
    let accounts: Vec<AccountMeta> = ctx.remaining_accounts
    .iter()
    .map(|acc| AccountMeta {
        pubkey: *acc.key,
        is_signer: acc.is_signer,
        is_writable: acc.is_writable,
    })
    .collect();
    
    // Clone ais for instruction ais
    let account_infos: Vec<AccountInfo> = ctx.remaining_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    // Swap to group output vault
    invoke(
        &Instruction {
            program_id: *ctx.accounts.jupiter_program.key,
            accounts,
            data,
        },
        &account_infos,
    )?;

    // Get the amount of whatever output token group has after the swap takes place
    ctx.accounts.group_output_vault.reload()?;
    let bal_post = ctx.accounts.group_output_vault.amount;

    // Figure out fee-eligible amount
    let fee_eligible = bal_post - bal_pre;

    require!(fee_eligible > 0, KivoError::NegDelta);

    // Figure out what our fee actually is based on the delta
    let fee = fee_eligible / 200;

    // This is how much of whatever token is going to 
    let bal_delta = fee_eligible - fee;

    // Transfer our 0.5% fee to the Kivo Vault
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.group_output_vault.to_account_info(),
                to: ctx.accounts.kivo_vault.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            }
        ),
        fee as u64
    )?;

    // Transfer the rest to the receiver's output vault
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.group_output_vault.to_account_info(),
                to: ctx.accounts.output_vault.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            }
        ),
        bal_delta as u64
    )?;

    ctx.accounts.input_balance.decrement_balance(amt);
    msg!("Balance {} for mint {} and group {} owned by {} decreased by {}", 
        ctx.accounts.input_balance.key().to_string(), 
        ctx.accounts.input_mint.key().to_string(),
        ctx.accounts.group.key().to_string(),
        ctx.accounts.sender.key().to_string(),
        amt
    );

    msg!("{} of mint {} sent to {}",
        bal_delta,
        ctx.accounts.output_mint.key().to_string(),
        ctx.accounts.receiver.key.to_string(),
    );

    ctx.accounts.input_balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct SwapSplit<'info> {
    #[account(mut, associated_token::mint = input_mint, associated_token::authority = group)]
    pub group_input_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub kivo_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::mint = output_mint, associated_token::authority = group)]
    pub group_output_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::mint = output_mint, associated_token::authority = receiver)]
    pub output_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            sender.key().as_ref(),
            group.key().as_ref(),
            input_mint.key().as_ref(),
        ],
        bump
    )]
    pub input_balance: Box<Account<'info, Balance>>,

    pub sender: Box<Account<'info, User>>,

    /// CHECK: validated in output vault constraints
    pub receiver: UncheckedAccount<'info>,

    pub input_mint: Box<Account<'info, Mint>>,

    pub output_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub group: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub jupiter_program: Program<'info, Jupiter>,

    pub system_program: Program<'info, System>,
}