use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use anchor_spl::token::*;

use crate::state::contract::*;
use crate::state::user::User;

pub fn process(ctx: Context<SettleContractPayment>) -> Result<clockwork_sdk::state::ThreadResponse> {
    msg!("Settling contract payment");

    let obligor = &mut ctx.accounts.obligor;
    let obligor_user_account = &ctx.accounts.obligor_user_account;
    let obligor_token_account = &mut ctx.accounts.obligor_token_account;
    let contract = &mut ctx.accounts.contract;
    let contract_thread = &ctx.accounts.contract_thread;
    let receiver_token_account = &mut ctx.accounts.receiver_token_account;
    let thread_program = &ctx.accounts.thread_program;
    let token_program = &ctx.accounts.token_program;
    let _contract_creator = &ctx.accounts.contract_creator;

    let contract_key = contract.key();
    let obligor_user_account_key = obligor_user_account.key();
    let obligor_bump = obligor.bump;

    let signature_seeds = Obligor::get_obligor_signer_seeds(&obligor_user_account_key, &contract_key, &obligor_bump);
    let signer_seeds = &[&signature_seeds[..]];

    if contract.is_fulfilled() {
        msg!("Contract fulfilled - deleting Thread");

        let thread_delete_accounts = clockwork_sdk::cpi::ThreadDelete {
            authority: obligor.to_account_info(),
            close_to: obligor_token_account.to_account_info(),
            thread: contract_thread.to_account_info(),
        };

        let thread_delete_cpi_context = CpiContext::new_with_signer(
            thread_program.to_account_info(),
            thread_delete_accounts,
            signer_seeds,
        );

        clockwork_sdk::cpi::thread_delete(thread_delete_cpi_context)?;
    } 
    else {
        obligor.last_payment_at = Some(Clock::get().unwrap().unix_timestamp);

        let settle_contract_payment_accounts = Transfer {
            authority: obligor.to_account_info(),
            from: obligor_token_account.to_account_info(),
            to: receiver_token_account.to_account_info(),
        };

        let settle_contract_payment_cpi_context = CpiContext::new_with_signer(
            token_program.to_account_info(),
            settle_contract_payment_accounts,
            signer_seeds,
        );

        transfer(settle_contract_payment_cpi_context, contract.amount)?;

        contract.increment_payments_made();

        contract.exit(&crate::id())?;
        obligor.exit(&crate::id())?;
    }
    
    Ok(clockwork_sdk::state::ThreadResponse::default())
}

#[derive(Accounts)]
pub struct SettleContractPayment<'info> {

    #[account(mut, address = Obligor::get_obligor_address(obligor.user_account, contract.key()).0)]
    pub obligor: Box<Account<'info, Obligor>>,

    #[account()]
    pub obligor_user_account: Box<Account<'info, User>>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = obligor)]    
    pub obligor_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = Contract::get_contract_address(contract.obligor_user_account.key(), contract.id.clone()).0)]
    pub contract: Box<Account<'info, Contract>>,

    #[account(signer)]
    pub contract_thread: Box<Account<'info, clockwork_sdk::state::Thread>>,

    #[account()]
    pub contract_creator: Box<Account<'info, User>>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = contract.proposer_user_account)]    
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,
    
    // Add constraint here
    pub thread_program: Program<'info, clockwork_sdk::ThreadProgram>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}