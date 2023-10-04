use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
};
use anchor_spl::token::*;
use crate::state::{
    group::Balance,
    user::User
};
use crate::error::KivoError;
use spl_stake_pool::instruction::deposit_sol;

pub fn process(ctx: Context<Freeze>, amt: u64) -> Result<()> {
    require!(ctx.accounts.sol_balance.balance > amt, KivoError::BadWithdrawal);

    let init_bal = ctx.accounts.pool_tokens_to.amount;

    let instruction = deposit_sol(
        ctx.accounts.stake_pool_program.key,
        &ctx.accounts.stake_pool.key,
        ctx.accounts.stake_pool_withdraw_authority.key,
        ctx.accounts.reserve_stake_account.key,
        &ctx.accounts.group_sol_vault.key(),
        &ctx.accounts.pool_tokens_to.key(),
        ctx.accounts.manager_fee_account.key,
        ctx.accounts.referrer_pool_tokens_account.key,
        &ctx.accounts.lst_mint.key(),
        ctx.accounts.token_program.key,
        amt,
    );

    let cpi_accounts = &[
        ctx.accounts.stake_pool.clone(),
        ctx.accounts.stake_pool_withdraw_authority.clone(),
        ctx.accounts.reserve_stake_account.clone(),
        ctx.accounts.group_sol_vault.clone().to_account_info(),
        ctx.accounts.pool_tokens_to.clone().to_account_info(),
        ctx.accounts.manager_fee_account.clone(),
        ctx.accounts.referrer_pool_tokens_account.clone(),
        ctx.accounts.lst_mint.clone().to_account_info(),
        ctx.accounts.stake_pool_program.clone(),
        ctx.accounts.token_program.clone().to_account_info(),
        ctx.accounts.rent.clone().to_account_info(),
    ];

    invoke(&instruction, cpi_accounts)?;

    ctx.accounts.pool_tokens_to.reload()?;
    let post_bal = ctx.accounts.pool_tokens_to.amount;

    let bal_delta = init_bal - post_bal;

    require!(bal_delta > 0, KivoError::NegDelta);

    if !ctx.accounts.lst_balance.initialized {
        ctx.accounts.lst_balance.new(
            ctx.accounts.user_lamports_from.key(),
            ctx.accounts.group.key(),
            ctx.accounts.lst_mint.key(),
        )?;
    }

    ctx.accounts.sol_balance.decrement_balance(amt);
    msg!("Balance {} for mint {} and group {} owned by {} decreased by {}",
        ctx.accounts.sol_balance.key().to_string(),
        spl_token::native_mint::id().to_string(),
        ctx.accounts.group.key().to_string(),
        ctx.accounts.user_lamports_from.key().to_string(),
        amt
    );

    ctx.accounts.lst_balance.increment_balance(bal_delta);
    msg!("Balance {} for mint {} and group {} owned by {} increased by {}",
        ctx.accounts.lst_balance.key().to_string(),
        ctx.accounts.lst_mint.key().to_string(),
        ctx.accounts.group.key().to_string(),
        ctx.accounts.user_lamports_from.key().to_string(),
        amt
    );

    ctx.accounts.sol_balance.exit(&crate::id())?;
    ctx.accounts.lst_balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(address = User::get_user_address(payer.key()).0)]
    pub user_lamports_from: Box<Account<'info, User>>,

    #[account(associated_token::mint = spl_token::native_mint::id(), associated_token::authority = group)]
    pub group_sol_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            user_lamports_from.key().as_ref(),
            group.key().as_ref(),
            spl_token::native_mint::id().as_ref(),
        ],
        bump
    )]
    pub sol_balance: Box<Account<'info, Balance>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = std::mem::size_of::<Balance>() + 8,
        seeds = [
            user_lamports_from.key().as_ref(),
            group.key().as_ref(),
            lst_mint.key().as_ref(),
        ],
        bump
    )]
    pub lst_balance: Box<Account<'info, Balance>>,

    pub lst_mint: Box<Account<'info, Mint>>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    pub stake_pool_withdraw_authority: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub reserve_stake_account: AccountInfo<'info>,

    #[account(mut)]
    pub pool_tokens_to: Box<Account<'info, TokenAccount>>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub manager_fee_account: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub referrer_pool_tokens_account: AccountInfo<'info>,

    #[account(mut)]
    pub group: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Checked in constraints
    #[account(address = spl_stake_pool::ID)]
    pub stake_pool_program: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}