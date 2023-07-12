use anchor_lang::prelude::*;
use anchor_lang::solana_program::{ system_program };
use anchor_spl::token::*;
use anchor_spl::associated_token::*;
use std::mem::size_of;
use clockwork_sdk::state::Thread;
use clockwork_sdk::ThreadProgram;

use crate::state::user::User;
use crate::state::contract::*;

pub const USER: &[u8] = b"user";
pub const CONTRACT: &[u8] = b"contract";
pub const OBLIGOR: &[u8] = b"obligor";
pub const PROPOSAL: &[u8] = b"proposal";

#[derive(Accounts)]
pub struct ProposeContract<'info> {
    #[account(
        init, 
        payer = payer,
        space = 8 + size_of::<Contract>(),
        seeds = [
            CONTRACT,
            sender_user_account.key().as_ref(),
            sender_user_account.num_contracts.to_le_bytes().as_ref(),
            ],
        bump,
        )]
    pub contract: Box<Account<'info, Contract>>,

    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<Proposal>(),
        seeds = [
            PROPOSAL,
            receiver_user_account.key().as_ref(),
            receiver_user_account.num_proposals.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub proposal: Box<Account<'info, Proposal>>,

    #[account(mut)]
    pub sender_user_account: Box<Account<'info, User>>,

    #[account(associated_token::mint = mint, associated_token::authority = sender_user_account)]    
    pub sender_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub receiver_user_account: Box<Account<'info, User>>,

    #[account(associated_token::mint = mint, associated_token::authority = receiver_user_account)]    
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct AcceptContract<'info> {
    #[account(mut, address = Contract::get_contract_address(contract.receiver.key(), contract.nonce.clone()).0)]
    pub contract: Box<Account<'info, Contract>>,

    #[account(mut, address = contract.receiver.key())]
    pub contract_creator: Box<Account<'info, User>>, 

    #[account(mut)]
    pub obligor_user_account: Box<Account<'info, User>>,

    #[account(
        init, 
        seeds = [
            OBLIGOR,
            payer.key().as_ref(),
            contract.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Obligor>(),
    )]
    pub obligor: Box<Account<'info, Obligor>>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = obligor_user_account)]    
    pub obligor_token_account: Box<Account<'info, TokenAccount>>, // this is the same as contract.sender_token_account

    #[account(mut, associated_token::mint = mint, associated_token::authority = contract_creator)]    
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,
    
    /// CHECK: Thread initialized via CPI
    #[account(mut, address = Thread::pubkey(contract.sender.key(), contract.id.clone().into_bytes()))]
    pub contract_thread: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    // Add Thread Program ID
    pub thread_program: Program<'info, ThreadProgram>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
    
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RejectContract<'info> {
    #[account(mut, address = Contract::get_contract_address(contract.receiver.key(), contract.nonce.clone()).0)]
    pub contract: Account<'info, Contract>,

    #[account(address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}