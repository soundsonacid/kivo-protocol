use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        program::invoke_signed,
    }
};
use anchor_spl::token::*;
use crate::state::{
    group::{
        Group,
        Balance
    },
    user::User
};
use crate::error::KivoError;
use spl_stake_pool::instruction::StakePoolInstruction::DepositSol;

pub fn process(ctx: Context<Freeze>, amt: u64) -> Result<()> {

    require!(ctx.accounts.sol_balance.balance > amt, KivoError::BadWithdrawal);

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
            program_id: spl_stake_pool::id(),
            accounts,
            data: DepositSol(amt).try_to_vec().unwrap(),
        },
        &account_infos,
        signer_seeds,
    )?;

    if !ctx.accounts.lst_balance.initialized {
        ctx.accounts.lst_balance.new(
            ctx.accounts.user.key(),
            ctx.accounts.group.key(),
            ctx.accounts.lst_mint.key(),
        )?;
    }

    ctx.accounts.sol_balance.decrement_balance(amt);
    // This won't work right yet - need to use output amount instead of input.
    ctx.accounts.lst_balance.increment_balance(amt);

    ctx.accounts.sol_balance.exit(&crate::id())?;
    ctx.accounts.lst_balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    pub group: Account<'info, Group>,

    #[account(address = User::get_user_address(payer.key()).0)]
    pub user: Account<'info, User>,

    #[account(associated_token::mint = spl_token::native_mint::id(), associated_token::authority = group)]
    pub group_sol_vault: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            user.key().as_ref(),
            group.key().as_ref(),
            spl_token::native_mint::id().as_ref(),
        ],
        bump
    )]
    pub sol_balance: Account<'info, Balance>,

    #[account(
        init_if_needed,
        payer = payer,
        space = std::mem::size_of::<Balance>() + 8,
        seeds = [
            user.key().as_ref(),
            group.key().as_ref(),
            lst_mint.key().as_ref(),
        ],
        bump
    )]
    pub lst_balance: Account<'info, Balance>,

    pub lst_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}