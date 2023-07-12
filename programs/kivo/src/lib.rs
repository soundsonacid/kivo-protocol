use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::hash;
use clockwork_sdk::state::ThreadResponse;
use clockwork_sdk::cpi::{ ThreadCreate, thread_create, ThreadDelete, thread_delete };
use clockwork_sdk::state::Trigger;

use crate::instructions::user::*;
use crate::instructions::transaction::*;
use crate::instructions::contract_manager::*;
use crate::instructions::contract_controller::*;

use crate::state::user::*;
use crate::state::contract::Obligor;

pub mod state;
pub mod instructions;

declare_id!("7bRUosmoUkYVgZJHj2UDWM6kgHoy748R6NGweiDEk2vZ");

#[program]
pub mod kivo {
    use super::*;

    pub fn handle_initialize_user(ctx: Context<InitializeUser>, 
                                            name: [u8; 16], 
                                            account_type: u8) -> Result<()> {
        msg!("Initalizing user!");
    
        require!(name.iter().all(|&value| (value >= 97 && value <= 122) || (value >= 48 && value <= 57) || (value == 0)), KivoError::InvalidUsername);

        let user = &mut ctx.accounts.user_account;
        let username = &mut ctx.accounts.username_account;
    
        username.new(
            user.key(),
            name,
        )?;
    
        user.new(
            ctx.accounts.owner.clone().key(),
            name,
            account_type,
        )?;
        
        username.exit(&crate::id())?;
        user.exit(&crate::id())?;
    
        Ok(())
    }

    pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!("Depositing");

        let deposit_accounts = Transfer {
            from: ctx.accounts.depositor_token_account.to_account_info(),
            to: ctx.accounts.pda_token_account.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info().clone(),
        };

        let token_program = ctx.accounts.token_program.to_account_info().clone();

        let deposit_cpi_context = CpiContext::new(token_program, deposit_accounts);

        transfer(deposit_cpi_context, amount)?;

        Ok(())
    }

    pub fn handle_withdrawal(ctx: Context<Withdrawal>, amount: u64, bump: u8) -> Result<()> {
        msg!("Withdrawing");
    
        let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];    
    
        let token_program = ctx.accounts.token_program.to_account_info().clone();

        let transfer_accounts = Transfer {
            from: ctx.accounts.pda_token_account.to_account_info(),
            to: ctx.accounts.withdrawer_token_account.to_account_info(),
            authority: ctx.accounts.user_account.to_account_info().clone(),
        };
        let cpi_ctx_transfer = CpiContext::new_with_signer(
            token_program.to_account_info().clone(),
            transfer_accounts,
            signer_seeds,
        );
        transfer(cpi_ctx_transfer, amount)?;
    
        ctx.accounts.user_account.increment_withdrawals();
    
        ctx.accounts.user_account.exit(&crate::id())?;
    
        Ok(())
    }
    
    pub fn handle_unwrap_withdrawal(ctx: Context<UnwrapWithdrawal>, amount: u64, bump: u8) -> Result<()> {
        msg!("Unwrapping & withdrawing");

        let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];   

        let token_program = ctx.accounts.token_program.to_account_info().clone();

        let transfer_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.temporary_token_account.to_account_info(),
            authority: ctx.accounts.user_account.to_account_info().clone(),
        };

        let cpi_ctx_transfer = CpiContext::new_with_signer(
            token_program.to_account_info().clone(),
            transfer_accounts,
            signer_seeds,
        );

        transfer(cpi_ctx_transfer, amount)?;

        let close_accounts = CloseAccount {
            account: ctx.accounts.temporary_token_account.to_account_info().clone(),
            destination: ctx.accounts.withdrawer.to_account_info().clone(),
            authority: ctx.accounts.user_account.to_account_info().clone(),
        };

        let cpi_ctx_close = CpiContext::new_with_signer(
            token_program.to_account_info().clone(),
            close_accounts,
            signer_seeds,
        );

        close_account(cpi_ctx_close)?;

        ctx.accounts.user_account.increment_withdrawals();
    
        ctx.accounts.user_account.exit(&crate::id())?;

        Ok(())
    }
    
    pub fn handle_execute_transaction(ctx: Context<ExecuteTransaction>, 
                                                amount: u64, 
                                                bump: u8, 
                                                time_stamp: u64) -> Result<()> {
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

    pub fn handle_create_request(ctx: Context<CreateRequest>, 
                                            amount: u64, 
                                            time_stamp: u64) -> Result<()> {
        msg!("Creating request");

        let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;
        let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
        let requester = &mut ctx.accounts.requester;
        let fulfiller = &mut ctx.accounts.fulfiller;
        let mint = &ctx.accounts.mint;

        requester_transaction_account.new(
            requester.key(),
            requester.username.clone(),
            mint.key(), 
            amount, 
            time_stamp, 
            fulfiller.key(),
            fulfiller.username.clone(),
            fulfiller_transaction_account.key(), 
            false, 
        )?;

        requester_transaction_account.exit(&crate::id())?;

        let fulfiller_transaction_account_key = fulfiller_transaction_account.key();

        fulfiller_transaction_account.new(
            requester.key(),
            requester.username.clone(),
            mint.key(),
            amount,
            time_stamp,
            fulfiller.key(),
            fulfiller.username.clone(),
            fulfiller_transaction_account_key,
            false,
        )?;

        fulfiller_transaction_account.exit(&crate::id())?;

        requester.increment_transactions();
        fulfiller.increment_transactions();

        requester.exit(&crate::id())?;
        fulfiller.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_fulfill_request(ctx: Context<FulfillRequest>, 
                                            amount: u64, 
                                            bump: u8) -> Result<()> {
        msg!("Fulfilling transaction!");

        let fulfiller = &ctx.accounts.fulfiller;
        let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
        let requester = &ctx.accounts.requester;
        let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;

        let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];

        let request_accounts = Transfer {
            from: ctx.accounts.fulfiller_token_account.to_account_info(),
            to: ctx.accounts.requester_token_account.to_account_info(),
            authority: fulfiller.to_account_info()
        };

        let token_program = ctx.accounts.token_program.to_account_info();

        let request_cpi_context = CpiContext::new_with_signer(token_program, request_accounts, signer_seeds);

        transfer(request_cpi_context, amount)?;

        fulfiller_transaction_account.fulfill(
            fulfiller.key(),
            fulfiller.username.clone(),
            requester.key(),
            requester.username.clone(),
            true
        )?;

        requester_transaction_account.fulfill(
            fulfiller.key(),
            fulfiller.username.clone(),
            requester.key(),
            requester.username.clone(),
            true
        )?;

        fulfiller_transaction_account.exit(&crate::id())?;
        requester_transaction_account.exit(&crate::id())?;
        
        Ok(())
    }

    pub fn handle_edit_username(ctx: Context<EditUsername>, name: [u8; 16]) -> Result<()> {
        msg!("Editing username");
    
        require!(name.iter().all(|&value| (value >= 97 && value <= 122) || (value >= 48 && value <= 57) || (value == 0)), KivoError::InvalidUsername);

        let new_username = &mut ctx.accounts.new_username_account;
        let user = &mut ctx.accounts.user_account;
        
        ctx.accounts.old_username_account.close(user.to_account_info())?;
    
        new_username.new(
            user.key(),
            name,
        )?;
    
        user.set_username(name);
    
        user.exit(&crate::id())?;
        new_username.exit(&crate::id())?;
        
        Ok(())
    }

    pub fn handle_set_preferred_token(ctx: Context<SetPreferredToken>) -> Result<()> {
        msg!("Setting preferred token");

        let user = &mut ctx.accounts.user_account;
        let new_preferred_token = &ctx.accounts.preferred_token_mint;

        user.set_preferred_token(new_preferred_token.key());

        user.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_disable_preferred_token(ctx: Context<DisablePreferredToken>) -> Result<()> {
        msg!("Disabling preferred token");

        let user = &mut ctx.accounts.user_account;

        user.disable_preferred_token();

        user.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_add_friend(ctx: Context<AddFriend>) -> Result<()> {
        msg!("Adding friend");

        let user = &mut ctx.accounts.user_account;
        let friend = &ctx.accounts.friend_account;
        let friend_account = &mut ctx.accounts.new_friend;

        friend_account.new(
            friend.key(),
            friend.username.clone(),
            friend.account_type.clone(),
        )?;

        user.increment_friends();
        user.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_propose_contract(ctx: Context<ProposeContract>, amount: u64, schedule: String, id: String, bump: u8, num_payments_obligated: u64) -> Result<()> {
        msg!("Proposing contract");

        let contract = &mut ctx.accounts.contract;
        let proposal = &mut ctx.accounts.proposal;
        let sender = &mut ctx.accounts.sender_user_account;
        let sender_token_account = &ctx.accounts.sender_token_account;
        let receiver = &mut ctx.accounts.receiver_user_account;
        let receiver_token_account = &ctx.accounts.receiver_token_account;

        let id_clone = id.clone();
        let sched_clone = schedule.clone();

        contract.new(
            sender.key(),
            sender_token_account.key(),
            receiver.key(),
            receiver_token_account.key(),
            amount,
            schedule,
            id,
            bump,
            num_payments_obligated,
            sender.num_contracts.clone(),
            proposal.key(),
        )?;

        proposal.new(
            sender.key(),
            sender.username.clone(),
            sched_clone,
            num_payments_obligated.clone(),
            id_clone,
            amount,
            contract.key(),
            receiver.num_proposals,
        )?;

        receiver.increment_proposals();
        receiver.increment_contracts();
        sender.increment_contracts();

        sender.exit(&crate::id())?;
        receiver.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_accept_contract(ctx: Context<AcceptContract>, obligor_bump: u8, user_bump: u8) -> Result<()> {
        msg!("Accepting contract");
        
        let contract = &mut ctx.accounts.contract;
        let proposal = &mut ctx.accounts.proposal;
        let obligor = &mut ctx.accounts.obligor;
        let obligor_user_account = &mut ctx.accounts.obligor_user_account;

        let obligor_token_account = &mut ctx.accounts.obligor_token_account;

        require!(obligor_token_account.amount >= contract.amount, KivoError::InsufficientBalanceToAcceptContract);
        require!(contract.sender.key() == ctx.accounts.payer.key(), KivoError::BadSignerToAcceptContract);

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
        let contract_creator = &mut ctx.accounts.contract_creator;
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
                AccountMeta::new(contract_creator.key(), false),
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
        let obligor_signer_seeds = &[&obligor_signature_seeds[..]];

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

        // let thread_create_accounts = ThreadCreate {
        //     authority: payer.to_account_info(),
        //     payer: payer.to_account_info(),
        //     system_program: system_program.to_account_info(),
        //     thread: contract_thread.to_account_info(),
        // };

        let thread_create_accounts = ThreadCreate {
            authority: obligor.to_account_info(),
            payer: payer.to_account_info(),
            system_program: system_program.to_account_info(),
            thread: contract_thread.to_account_info(),
        };

        let thread_create_cpi_context = CpiContext::new_with_signer(
            thread_program.to_account_info(), 
            thread_create_accounts,
            obligor_signer_seeds,
        );

        let trigger = Trigger::Cron {
            schedule: contract.schedule.clone(),
            skippable: false,
        };

        thread_create(thread_create_cpi_context, LAMPORTS_PER_SOL / 100 as u64, contract.id.clone().as_bytes().to_vec(), vec![settle_contract_payment_ix.into()], trigger)?;

        msg!("Thread created");

        contract.accept(contract_thread.key());
        proposal.accept();

        contract_creator.exit(&crate::id())?;
        contract.exit(&crate::id())?;
        proposal.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_reject_contract(ctx: Context<RejectContract>) -> Result<()> {
        msg!("Rejecting contract");

        let contract = &mut ctx.accounts.contract;
        let proposal = &mut ctx.accounts.proposal;
        let authority = contract.sender;
        let user = &ctx.accounts.user_account;

        require!(authority == user.key(), KivoError::BadSignerToRejectContract);

        contract.close(ctx.accounts.payer.to_account_info())?;
        proposal.reject();

        proposal.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_settle_contract_payment(ctx: Context<SettleContractPayment>) -> Result<ThreadResponse> {
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

            let thread_delete_accounts = ThreadDelete {
                authority: obligor.to_account_info(),
                close_to: obligor_token_account.to_account_info(),
                thread: contract_thread.to_account_info(),
            };

            let thread_delete_cpi_context = CpiContext::new_with_signer(
                thread_program.to_account_info(),
                thread_delete_accounts,
                signer_seeds,
            );

            thread_delete(thread_delete_cpi_context)?;
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
        
        Ok(ThreadResponse::default())
    }
}

#[error_code]
pub enum KivoError {
    #[msg("Insufficient funds to accept contract!")]
    InsufficientBalanceToAcceptContract,
    #[msg("Failed to reject contract: Bad signer at handle_reject_contract - signer key must match contract.sender!")]
    BadSignerToRejectContract,
    #[msg("Failed to accept contract: Bad signer at handle_accept_contract - signer key must match contract.sender!")]
    BadSignerToAcceptContract,
    #[msg("Username contains invalid characters - Usernames must be 16 characters or less and all lowercase letters or numbers!")]
    InvalidUsername,
}