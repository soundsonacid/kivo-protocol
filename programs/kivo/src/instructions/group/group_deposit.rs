use crate::state::group::Balance;
use crate::state::user::User;
use anchor_lang::prelude::*;
use anchor_spl::token::*;

pub fn process(ctx: Context<DepositToGroupWallet>, deposit: u64) -> Result<()> {
    let bump = User::get_user_address(ctx.accounts.payer.key()).1;
    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_vault.to_account_info(),
                to: ctx.accounts.group_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
            signer_seeds,
        ),
        deposit
    )?;

    if !ctx.accounts.balance.initialized {
        ctx.accounts.balance.new(
            ctx.accounts.user.key(),
            ctx.accounts.group.key(),
            ctx.accounts.mint.key(),
        )?;
    }

    ctx.accounts.balance.increment_balance(deposit);
    msg!("Balance {} for mint {} and group {} owned by {} increased by {}",
        ctx.accounts.balance.key().to_string(),
        ctx.accounts.mint.key().to_string(),
        ctx.accounts.group.key().to_string(),
        ctx.accounts.user.key().to_string(),
        deposit
    );
    ctx.accounts.balance.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositToGroupWallet<'info> {
    /// CHECK: validated by vault ownership constraint & CPI
    pub group: UncheckedAccount<'info>,

    #[account(address = User::get_user_address(payer.key()).0)]
    pub user: Account<'info, User>,

    #[account(
        mut,
        associated_token::authority = group,
        associated_token::mint = mint,
    )]
    pub group_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = user,
        associated_token::mint = mint,
    )]
    pub user_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [
            user.key().as_ref(),
            group.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = std::mem::size_of::<Balance>() + 8
    )]
    pub balance: Account<'info, Balance>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}