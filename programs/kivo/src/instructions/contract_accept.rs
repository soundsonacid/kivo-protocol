use anchor_lang::{
    prelude::*,
    solana_program::{
        system_program,
        hash,
        native_token::LAMPORTS_PER_SOL,
        instruction::Instruction,
    },
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
        contract::Obligor,
    },
    constants::OBLIGOR,
    error::KivoError,
};

pub fn process(ctx: Context<AcceptContract>) -> Result<()> {
    msg!("Accepting contract");
    
    let contract = &mut ctx.accounts.contract;
    let proposal = &mut ctx.accounts.proposal;
    let obligor = &mut ctx.accounts.obligor;
    let obligor_user_account = &mut ctx.accounts.obligor_user_account;

    let obligor_token_account = &mut ctx.accounts.obligor_token_account;

    let obligor_bump = Obligor::get_obligor_address(obligor.key(), contract.key()).1;
    let user_bump = User::get_user_address(ctx.accounts.payer.key()).1;

    require!(obligor_token_account.amount >= contract.amount, KivoError::InsufficientBalanceToAcceptContract);
    require!(contract.obligor_user_account.key() == User::get_user_address(ctx.accounts.payer.key()).0, KivoError::BadSignerToAcceptContract);

    obligor.new(
        obligor_user_account.key(),
        contract.key(),
        obligor_bump,
    )?;

    obligor.exit(&crate::id())?;

    let contract_thread = &ctx.accounts.contract_thread;
    let receiver_token_account = &ctx.accounts.receiver_token_account;
    let thread_program = &ctx.accounts.thread_program;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &mut ctx.accounts.payer;
    let proposer = &mut ctx.accounts.proposer;
    let mint = &ctx.accounts.mint;

    let mut discriminator = [0u8; 8];
    let preimage = format!("{}:{}", "global", "SettleContractPayment");
    let hash_result = &hash::hash(preimage.as_bytes());
    discriminator.copy_from_slice(&hash_result.to_bytes()[..8]);

    let settle_contract_payment_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(obligor.key(), false),
            AccountMeta::new(obligor_user_account.key(), false),
            AccountMeta::new(obligor_token_account.key(), false),
            AccountMeta::new(contract.key(), false),
            AccountMeta::new_readonly(contract_thread.key(), true),
            AccountMeta::new(proposer.key(), false),
            AccountMeta::new(receiver_token_account.key(), false),
            AccountMeta::new_readonly(mint.key(), false),
            AccountMeta::new_readonly(thread_program.key(), false),
            AccountMeta::new_readonly(token_program.key(), false),
            AccountMeta::new_readonly(system_program.key(), false),
        ],
        data: discriminator.into(),
    };

    msg!("Instruction built");

    let payer_key = payer.key();

    let contract_key = contract.key();
    let obligor_user_account_key = obligor_user_account.key();

    let user_signature_seeds = User::get_user_signer_seeds(&payer_key, &user_bump);
    let user_signer_seeds = &[&user_signature_seeds[..]];

    let obligor_signature_seeds = Obligor::get_obligor_signer_seeds(&obligor_user_account_key, &contract_key, &obligor_bump);
    let _obligor_signer_seeds = &[&obligor_signature_seeds[..]];

    let delegate_accounts = Approve {
        authority: obligor_user_account.to_account_info(),
        delegate: obligor.to_account_info(),
        to: obligor_token_account.to_account_info()
    };

    let delegate_cpi_context = CpiContext::new_with_signer(
        token_program.to_account_info(),
        delegate_accounts,
        user_signer_seeds,
    );

    approve(delegate_cpi_context, u64::MAX)?;

    msg!("Delegate approved");

    let thread_create_accounts = clockwork_sdk::cpi::ThreadCreate {
        authority: payer.to_account_info(),
        payer: payer.to_account_info(),
        system_program: system_program.to_account_info(),
        thread: contract_thread.to_account_info(),
    };

    // let thread_create_accounts = ThreadCreate {
    //     authority: obligor.to_account_info(),
    //     payer: payer.to_account_info(),
    //     system_program: system_program.to_account_info(),
    //     thread: contract_thread.to_account_info(),
    // };

    // let thread_create_cpi_context = CpiContext::new_with_signer(
    //     thread_program.to_account_info(), 
    //     thread_create_accounts,
    //     obligor_signer_seeds,
    // );

    let thread_create_cpi_context = CpiContext::new(
        thread_program.to_account_info(), 
        thread_create_accounts,
    );

    let trigger = clockwork_sdk::state::Trigger::Cron {
        schedule: contract.schedule.clone(),
        skippable: false,
    };

    clockwork_sdk::cpi::thread_create(thread_create_cpi_context, LAMPORTS_PER_SOL / 100 as u64, contract.id.clone().to_le_bytes().to_vec(), vec![settle_contract_payment_ix.into()], trigger)?;

    msg!("Thread created");

    contract.accept(contract_thread.key());
    proposal.accept();

    proposer.exit(&crate::id())?;
    contract.exit(&crate::id())?;
    proposal.exit(&crate::id())?;

    Ok(())
}

#[derive(Accounts)]
pub struct AcceptContract<'info> {
    #[account(mut, address = Contract::get_contract_address(contract.obligor_user_account.key(), contract.id.clone()).0)]
    pub contract: Box<Account<'info, Contract>>,

    #[account(mut, address = Proposal::get_proposal_address(contract.proposer_user_account.key(), proposal.nonce.clone()).0)]
    pub proposal: Box<Account<'info, Proposal>>,

    #[account(mut, address = contract.proposer_user_account.key())]
    pub proposer: Box<Account<'info, User>>, 

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
        space = 8 + std::mem::size_of::<Obligor>(),
    )]
    pub obligor: Box<Account<'info, Obligor>>,

    #[account(mut, associated_token::mint = mint, associated_token::authority = obligor_user_account)]    
    pub obligor_token_account: Box<Account<'info, TokenAccount>>, 

    #[account(mut, associated_token::mint = mint, associated_token::authority = proposer)]    
    pub receiver_token_account: Box<Account<'info, TokenAccount>>,
    
    /// CHECK: Thread initialized via CPI
    #[account(mut, address = clockwork_sdk::state::Thread::pubkey(payer.key(), contract.id.to_le_bytes().to_vec()))]
    pub contract_thread: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    // Add Thread Program ID
    pub thread_program: Program<'info, clockwork_sdk::ThreadProgram>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
    
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}