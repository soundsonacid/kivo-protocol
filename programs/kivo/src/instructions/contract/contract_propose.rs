use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use anchor_spl::{
    token::*,
    associated_token::*,
};
use crate::{
    state::{
        user::User,
        contract::Contract,
        contract::Proposal,
        transaction::Transaction,
    },
    constants::{CONTRACT, PROPOSAL},
};

pub fn process(ctx: Context<ProposeContract>, amount: u64, num_payments_obligated: u32) -> Result<()> {
    msg!("Proposing contract");

    let contract = &mut ctx.accounts.contract;
    let proposal = &mut ctx.accounts.proposal;
    let obligor = &mut ctx.accounts.obligor_user_account;
    let proposer = &mut ctx.accounts.proposer_user_account;
    let mint = &ctx.accounts.mint;

    let mint_id = Transaction::get_mint_id(&mint.key());

    let bump = Contract::get_contract_address(proposer.key(), proposer.num_contracts.clone()).1;

    contract.new(
        obligor.key(),
        proposer.key(),
        proposal.key(),
        mint_id,
        amount,
        bump,
        num_payments_obligated,
        obligor.num_contracts.clone(),
    )?;

    proposal.new(
        obligor.key(),
        num_payments_obligated.clone(),
        amount,
        contract.key(),
        mint_id,
        proposer.num_proposals,
    )?;

    proposer.increment_proposals();
    obligor.increment_contracts();

    proposer.exit(&crate::id())?;
    obligor.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct ProposeContract<'info> {
    #[account(
        init, 
        payer = payer,
        space = 8 + std::mem::size_of::<Contract>(),
        seeds = [
            CONTRACT,
            obligor_user_account.key().as_ref(),
            obligor_user_account.num_contracts.to_le_bytes().as_ref(),
            ],
        bump,
        )]
    pub contract: Box<Account<'info, Contract>>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Proposal>(),
        seeds = [
            PROPOSAL,
            proposer_user_account.key().as_ref(),
            proposer_user_account.num_proposals.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub proposal: Box<Account<'info, Proposal>>,

    #[account(mut)]
    pub obligor_user_account: Box<Account<'info, User>>,

    #[account(associated_token::mint = mint, associated_token::authority = obligor_user_account)]    
    pub obligor_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub proposer_user_account: Box<Account<'info, User>>,

    #[account(associated_token::mint = mint, associated_token::authority = proposer_user_account)]    
    pub proposer_token_account: Box<Account<'info, TokenAccount>>,

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