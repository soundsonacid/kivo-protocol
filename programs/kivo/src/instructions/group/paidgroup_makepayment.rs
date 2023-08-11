use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::token::*;
use crate::state::{
    user::User,
    group::PaidGroup,
};

pub fn process(ctx: Context<PaidGroupMakePayment>) -> Result<clockwork_sdk::state::ThreadResponse> {

    let bump = User::get_user_address(ctx.accounts.member.key()).1;
    let key = &ctx.accounts.member.key();
    let signature_seeds = User::get_user_signer_seeds(key, &bump);
    let signer_seeds = &[&signature_seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.member.to_account_info(),
                from: ctx.accounts.member_token_account.to_account_info(),
                to: ctx.accounts.destination_token_account.to_account_info(),
            },
            signer_seeds,
        ),
        ctx.accounts.group.fee
    )?;

    Ok(clockwork_sdk::state::ThreadResponse::default())
}

#[derive(Accounts)]
pub struct PaidGroupMakePayment<'info> {
    pub member: Account<'info, User>,

    #[account(mut)]
    pub member_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,

    pub group: Account<'info, PaidGroup>,

    pub thread_program: Program<'info, clockwork_sdk::ThreadProgram>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

