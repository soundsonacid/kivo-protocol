use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::*;
use clockwork_sdk::state::Thread;
use clockwork_sdk::ThreadProgram;

use crate::state::contract::*;
use crate::state::user::User;

#[derive(Accounts)]
pub struct SettleContractPayment<'info> {

    #[account(mut, address = Obligor::get_obligor_address(obligor.user_account, contract.key()).0)]
    pub obligor: Box<Account<'info, Obligor>>,

    #[account()]
    pub obligor_user_account: Box<Account<'info, User>>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = obligor)]    
    pub obligor_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = Contract::get_contract_address(contract.receiver.key(), contract.nonce.clone()).0)]
    pub contract: Box<Account<'info, Contract>>,

    #[account(signer)]
    pub contract_thread: Box<Account<'info, Thread>>,

    #[account()]
    pub contract_owner: Box<Account<'info, User>>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = contract.receiver)]    
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,
    
    // Add constraint here
    pub thread_program: Program<'info, ThreadProgram>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}