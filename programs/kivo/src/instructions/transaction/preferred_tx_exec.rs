use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        program::invoke,
    }
};
use anchor_spl::{token::*, associated_token::AssociatedToken};
use crate::{state::{
    user::User, 
    transaction::Transaction
}, constants::{OUTGOING, INCOMING, ZERO}};
use crate::error::KivoError;
use crate::jupiter::Jupiter;

pub fn process(ctx: Context<PreferredSwapExec>, amt: u64, data: Vec<u8>) -> Result<()> {
    let bal_pre = ctx.accounts.output_vault.amount;
    msg!("Initial output vault balance: {}", bal_pre);

    let bump = User::get_user_address(ctx.accounts.payer.key()).1;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];

    // Transfer funds to the user's wallet to be swapped
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                to: ctx.accounts.input_vault.to_account_info(),
                from: ctx.accounts.source_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info()
            },
            signer_seeds
        ),
        amt
    )?;

    msg!("Transferred {} of mint {} to {}",
        amt,
        ctx.accounts.input_mint.key().to_string(),
        ctx.accounts.input_vault.key().to_string(),
    );

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

    // Swap
    // source is the sender's wallet input mint vault
    // destination is the sender's user account output vault
    invoke(
        &Instruction {
            program_id: *ctx.accounts.jupiter_program.key,
            accounts,
            data,
        },
        &account_infos,
    )?;

    ctx.accounts.output_vault.reload()?;
    let bal_post = ctx.accounts.output_vault.amount;
    msg!("Final output vault balance: {}", bal_post);

    let bal_delta = bal_post - bal_pre;
    msg!("Balance delta: {}", bal_delta);

    if bal_delta.le(&ZERO) {
        msg!("Balance change for token {} in vault {} is LTE zero", 
            ctx.accounts.output_vault.mint.to_string(),
            ctx.accounts.output_vault.key().to_string()
        );
        msg!("Initial balance: {}", bal_pre);
        msg!("Final balance: {}", bal_post);
        msg!("Balance delta: {}", bal_delta);
        return Err(error!(KivoError::NegDelta));
    }

    let fee = bal_delta / 200;

    let amt_final = bal_delta - fee;

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.output_vault.to_account_info(),
                to: ctx.accounts.kivo_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
            signer_seeds,
        ),
        fee
    )?;

    msg!("{} of mint {} sent to {}",
        fee,
        ctx.accounts.output_vault.mint.to_string(),
        ctx.accounts.kivo_vault.key().to_string(),
    );

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.output_vault.to_account_info(),
                to: ctx.accounts.destination_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
            signer_seeds,
        ),
        amt_final,
    )?;

    msg!("{} of mint {} sent to {}",
        amt_final,
        ctx.accounts.output_vault.mint.to_string(),
        ctx.accounts.output_vault.key().to_string(),
    );

    ctx.accounts.payer_tx_account.new(
        ctx.accounts.destination_owner.key(),
        ctx.accounts.user.key(),
        amt_final,
        Some(true)
    )?;

    ctx.accounts.receiver_tx_account.new(
        ctx.accounts.destination_owner.key(),
        ctx.accounts.user.key(),
        amt_final,
        Some(true)
    )?;

    ctx.accounts.user.increment_outgoing_transactions();
    ctx.accounts.destination_owner.increment_incoming_transactions();

    ctx.accounts.payer_tx_account.exit(&crate::id())?;
    ctx.accounts.receiver_tx_account.exit(&crate::id())?;

    ctx.accounts.user.exit(&crate::id())?;
    ctx.accounts.destination_owner.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct PreferredSwapExec<'info> {
    #[account(mut)]
    pub user: Box<Account<'info, User>>,

    /// CHECK: validated in CPI
    #[account(mut)]
    pub source_vault: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = input_mint,
        associated_token::authority = payer
    )]
    pub input_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub output_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub destination_owner: Box<Account<'info, User>>,

    /// CHECK: validated in CPI
    #[account(mut)]
    pub destination_vault: UncheckedAccount<'info>,

    /// CHECK: validated in CPI
    #[account(mut)]
    pub kivo_vault: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            OUTGOING,
            user.to_account_info().key.as_ref(),
            user.outgoing_tx.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub payer_tx_account: Box<Account<'info, Transaction>>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            INCOMING,
            destination_owner.to_account_info().key.as_ref(),
            destination_owner.outgoing_tx.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub receiver_tx_account: Box<Account<'info, Transaction>>,

    pub input_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub jupiter_program: Program<'info, Jupiter>,

    pub system_program: Program<'info, System>,
}