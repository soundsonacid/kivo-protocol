use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};
use crate::{
    state::{
        user::User,
        contract::Contract,
        contract::Proposal,
    },
    error::KivoError,
};
pub fn process(ctx: Context<RejectContract>) -> Result<()> {
    msg!("Rejecting contract");

    let contract = &mut ctx.accounts.contract;
    let proposal = &mut ctx.accounts.proposal;
    let authority = contract.obligor_user_account;
    let user = &ctx.accounts.user_account;

    require!(authority == user.key(), KivoError::BadSignerToRejectContract);

    contract.close(user.to_account_info())?;
    proposal.reject();

    proposal.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct RejectContract<'info> {
    #[account(mut, address = Contract::get_contract_address(contract.obligor_user_account.key(), contract.id.clone()).0)]
    pub contract: Account<'info, Contract>,

    #[account(mut, address = Proposal::get_proposal_address(contract.proposer_user_account.key(), proposal.nonce.clone()).0)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut, address = User::get_user_address(payer.key()).0)]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}