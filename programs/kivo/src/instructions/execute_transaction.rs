use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::token::*;
use crate::{
    constants::TRANSACTION,
    state::{
        user::User,
        transaction::Transaction,
    }
};

pub fn process(ctx: Context<ExecuteTransaction>, amount: u64, bump: u8, time_stamp: u64) -> Result<()> {
    msg!("Executing transaction");

    let sender_transaction_account = &mut ctx.accounts.sender_transaction_account;
    let receiver_transaction_account = &mut ctx.accounts.receiver_transaction_account;
    let sender = &mut ctx.accounts.sender_user_account;
    let receiver = &mut ctx.accounts.receiver_user_account;
    let mint = &ctx.accounts.mint;

    let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.sender.key, &bump);
    let signer_seeds = &[&signature_seeds[..]];

    let transaction_accounts = Transfer {
    from: ctx.accounts.sender_token_account.to_account_info(),
    to: ctx.accounts.receiver_token_account.to_account_info(),
    authority: sender.to_account_info().clone(),
    };

    let token_program = ctx.accounts.token_program.to_account_info().clone();

    let transaction_cpi_context = CpiContext::new_with_signer(token_program, transaction_accounts, signer_seeds);

    transfer(transaction_cpi_context, amount)?;

    sender_transaction_account.new(
        sender.key(),
        sender.username.clone(),
        mint.key(),
        amount,
        time_stamp,
        receiver.key(),
        receiver.username.clone(),
        receiver_transaction_account.key(),
        true,
    )?;

    receiver_transaction_account.new(
        sender.key(),
        sender.username.clone(),
        mint.key(),
        amount,
        time_stamp,
        receiver.key(),
        receiver.username.clone(),
        sender_transaction_account.key(),
        true,
    )?;

    receiver.increment_transactions();
    sender.increment_transactions();

    receiver.exit(&crate::id())?;
    sender.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    /// CHECK: validated by cpi signer seeds
    pub sender: UncheckedAccount<'info>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub sender_user_account: Box<Account<'info, User>>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            TRANSACTION,
            sender_user_account.to_account_info().key.as_ref(),
            sender_user_account.transactions.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub sender_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = sender_user_account, associated_token::mint = mint)]
    pub sender_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub receiver_user_account: Account<'info, User>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Transaction>(),
        seeds = [
            TRANSACTION,
            receiver_user_account.to_account_info().key.as_ref(),
            receiver_user_account.transactions.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub receiver_transaction_account: Box<Account<'info, Transaction>>,

    #[account(mut, associated_token::authority = receiver_user_account, associated_token::mint = mint)]
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}