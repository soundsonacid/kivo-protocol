use anchor_lang::prelude::*;
use anchor_spl::token::*;
use clockwork_sdk::state::ThreadResponse;

use crate::instructions::user::*;
use crate::instructions::transaction::*;
use crate::instructions::contract_creation::*;
use crate::instructions::contract_controller::*;
use crate::state::user::*;

pub mod state;
pub mod instructions;

declare_id!("HyA8SiVhkkYoidUuFkmVXWDgRtiiwQTy465GwH5m6XSw");

#[program]
pub mod kivo {
    use super::*;

    pub fn handle_initialize_user(ctx: Context<InitializeUser>, 
                                            name: [u8; 16], 
                                            account_type: u8) -> Result<()> {
        msg!("Initalizing user!");
    
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
        msg!("Creating transaction account");

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

    pub fn handle_edit_username(ctx: Context<EditUsername>, username: [u8; 16]) -> Result<()> {
        msg!("Editing username");
    
        let new_username = &mut ctx.accounts.new_username_account;
        let user = &mut ctx.accounts.user_account;
        
        ctx.accounts.old_username_account.close(user.to_account_info())?;
    
        new_username.new(
            user.key(),
            username,
        )?;
    
        user.set_username(username);
    
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

    pub fn handle_propose_contract(ctx: Context<ProposeContract>, amount: u64, schedule: String, id: u64, bump: u8) -> Result<()> {
        msg!("Proposing contract");

        let contract = &mut ctx.accounts.contract;
        let sender = &ctx.accounts.sender_user_account;
        let sender_token_account = &ctx.accounts.sender_token_account;
        let receiver = &ctx.accounts.receiver_user_account;
        let receiver_token_account = &ctx.accounts.receiver_token_account;

        contract.new(
            sender.key(),
            sender_token_account.key(),
            receiver.key(),
            receiver_token_account.key(),
            amount,
            schedule,
            id,
            bump,
        )?;

        Ok(())
    }

    pub fn handle_accept_contract(ctx: Context<AcceptContract>, bump: u8) -> Result<()> {
        msg!("Entering contract");

        let contract = &mut ctx.accounts.contract;
        let obligor = &mut ctx.accounts.obligor;
        let obligor_sender = contract.sender;

        obligor.new(
            obligor_sender,
            contract.key(),
            bump,
        )?;

        contract.enter();

        Ok(())
    }

    pub fn handle_reject_contract(ctx: Context<RejectContract>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let authority = contract.sender;
        let assert = &ctx.accounts.payer.key();

        if authority == *assert {
            contract.close(ctx.accounts.payer.to_account_info())?;
        }

        Ok(())
    }
}