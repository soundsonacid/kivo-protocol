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
use crate::constants::ZERO;
use crate::error::KivoError;
use crate::jupiter::Jupiter;

// Should work for both entering & exiting an ape mode
// Executes a standard swap on jupiter between two arbitrary tokens
pub fn process(ctx: Context<Ape>, amt: u64, data: Vec<u8>) -> Result<()> {

    if ctx.accounts.user_input_balance.balance.lt(&amt) {
        msg!("Overuse of token {}", ctx.accounts.input_mint.key().to_string());
        msg!("User balance: {}", ctx.accounts.user_input_balance.balance);
        msg!("Attemped usage: {}", amt);
        return Err(error!(KivoError::ModeUsageExceedsBalance));
    }

    // Get the amount of whatever output token group has before the swap takes place
    let bal_pre = ctx.accounts.group_output_vault.amount;
    msg!("Initial output vault balance: {}", bal_pre);

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

    // Swap - destination is the group_output_vault
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
    msg!("Final output vault balance: {}", bal_post);

    // Figure out fee-eligible amount
    let bal_delta = bal_post - bal_pre;
    msg!("Balance delta: {}", bal_delta);

    if bal_delta.le(&ZERO) {
        msg!("Balance change for token {} in vault {} is LTE zero", 
            ctx.accounts.output_mint.key().to_string(), 
            ctx.accounts.group_output_vault.key().to_string()
        );
        msg!("Initial output vault balance: {}", bal_pre);
        msg!("Final output vault balance: {}", bal_post);
        msg!("Balance delta: {}", bal_delta);
        return Err(error!(KivoError::NegDelta));
    }

    // Figure out what our fee actually is based on the delta
    let fee = bal_delta / 200;

    // This is how much of whatever token is going to 
    let final_amt = bal_delta - fee;

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
        fee
    )?;

    msg!("{} of mint {} sent to {}",
        fee,
        ctx.accounts.output_mint.key().to_string(),
        ctx.accounts.kivo_vault.key().to_string(),
    );

    if !ctx.accounts.user_output_balance.initialized {

        msg!("Initializing Balance for mint {} and user {}", 
            ctx.accounts.output_mint.key().to_string(), 
            ctx.accounts.user.key().to_string(),
        );

        ctx.accounts.user_output_balance.new(
            ctx.accounts.user.key(),
            ctx.accounts.group.key(),
            ctx.accounts.output_mint.key()
        )?;
    };

    ctx.accounts.user_input_balance.decrement_balance(amt);
    msg!("Balance {} for mint {} and group {} owned by {} decreased by {}", 
        ctx.accounts.user_input_balance.key().to_string(), 
        ctx.accounts.input_mint.key().to_string(),
        ctx.accounts.group.key().to_string(),
        ctx.accounts.user.key().to_string(),
        amt
    );

    ctx.accounts.user_output_balance.increment_balance(final_amt);
    msg!("Balance {} for mint {} and group {} owned by {} increased by {}",
        ctx.accounts.user_output_balance.key().to_string(),
        ctx.accounts.output_mint.key().to_string(),
        ctx.accounts.group.key().to_string(),
        ctx.accounts.user.key().to_string(),
        final_amt
    );

    ctx.accounts.user_input_balance.exit(&crate::id())?;
    ctx.accounts.user_output_balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct Ape<'info> {
    #[account(mut, associated_token::mint = input_mint, associated_token::authority = group)]
    pub group_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub kivo_vault: Box<Account<'info, TokenAccount>>,

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
    pub group: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub jupiter_program: Program<'info, Jupiter>,

    pub system_program: Program<'info, System>,
}