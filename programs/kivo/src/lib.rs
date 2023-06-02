use anchor_lang::prelude::*;
use anchor_spl::token::*;
use clockwork_sdk::state::ThreadResponse;

use crate::instructions::user::*;
use crate::instructions::transaction::*;
use crate::instructions::contract::*;
use crate::state::traits::*;
use crate::state::user::*;

pub mod state;
pub mod instructions;

declare_id!("HyA8SiVhkkYoidUuFkmVXWDgRtiiwQTy465GwH5m6XSw");

#[program]
pub mod kivo {
    use super::*;

    pub fn handle_initialize_user(ctx: Context<InitializeUser>, name: [u8; 16], account_type: u8) -> Result<()> {
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

        let cpi_accounts = Transfer {
            from: ctx.accounts.depositor_token_account.to_account_info(),
            to: ctx.accounts.pda_token_account.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn handle_withdrawal(ctx: Context<Withdrawal>, amount: u64, bump: u8) -> Result<()> {
        msg!("Withdrawing");

        let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.pda_token_account.to_account_info(),
            to: ctx.accounts.withdrawer_token_account.to_account_info(),
            authority: ctx.accounts.user_account.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn handle_execute_transaction(ctx: Context<ExecuteTransaction>, amount: u64, bump: u8, time_stamp: u64) -> Result<()> {
        msg!("Executing transaction");

        let sender_transaction_account = &mut ctx.accounts.sender_transaction_account;
        let receiver_transaction_account = &mut ctx.accounts.receiver_transaction_account;
        let sender = &mut ctx.accounts.sender_user_account;
        let receiver = &mut ctx.accounts.receiver_user_account;
        let mint = &ctx.accounts.mint;

        let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: sender.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, amount)?;

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

        sender.increment_transactions();

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

        Ok(())
    }

    pub fn handle_create_transaction_account(ctx: Context<CreateTransactionAccount>, 
                                            amount: u64, 
                                            time_stamp: u64) -> Result<()> {
        msg!("Creating transaction account");

        let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;
        let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
        let requester = &mut ctx.accounts.requester;
        let fulfiller = &mut ctx.accounts.fulfiller;
        let mint = &mut ctx.accounts.mint;

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

        requester.increment_transactions();

        fulfiller_transaction_account.new(
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

        fulfiller.increment_transactions();

        requester.exit(&crate::id())?;
        requester_transaction_account.exit(&crate::id())?;
        fulfiller.exit(&crate::id())?;
        fulfiller_transaction_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn fulfill_transaction(ctx: Context<FulfillTransaction>, amount: u64, bump: u8) -> Result<()> {
        msg!("Fulfilling transaction!");

        let fulfiller = &ctx.accounts.fulfiller;
        let fulfiller_transaction_account = &mut ctx.accounts.fulfiller_transaction_account;
        let requester = &ctx.accounts.requester;
        let requester_transaction_account = &mut ctx.accounts.requester_transaction_account;

        let signature_seeds = User::get_user_signer_seeds(&ctx.accounts.payer.key, &bump);
        let signer_seeds = &[&signature_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.fulfiller_token_account.to_account_info(),
            to: ctx.accounts.requester_token_account.to_account_info(),
            authority: fulfiller.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, amount)?;

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

    pub fn handle_add_friend(ctx: Context<AddFriend>) -> Result<()> {
        msg!("Adding friend");

        let user = &mut ctx.accounts.user_account;
        let friend = &mut ctx.accounts.friend_account;
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

    pub fn handle_create_payment(ctx: Context<CreatePayment>, amount: u64, bump: u8) -> Result<()> {
        msg!("Creating payment");

        let user = &mut ctx.accounts.user_account;
        let user_token_account = &mut ctx.accounts.user_token_account;
        let mint = &ctx.accounts.mint;
        let payment = &mut ctx.accounts.payment;
        let receipient = &ctx.accounts.receipient;
        let token_program = &ctx.accounts.token_program;

        payment.new(
            amount,
            user.key(),
            mint.key(),
            receipient.key(),
        )?;

        approve(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Approve {
                    authority: user.to_account_info(),
                    to: user_token_account.to_account_info(),
                    delegate: payment.to_account_info(),
                }, 
                &[&[
                    b"user",
                    ctx.accounts.sender.key.as_ref(),
                    &[bump]]
                ]),
            u64::MAX
        )?;

        user.increment_contracts();
        user.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_disburse_payment(ctx: Context<DisbursePayment>) -> Result<ThreadResponse> {
        msg!("Disbursing payment");

        let user_token_account = &mut ctx.accounts.user_token_account;
        let payment = &mut ctx.accounts.payment;
        let receipient_token_account = &ctx.accounts.receipient;
        let token_program = &ctx.accounts.token_program;

        let bump = *ctx.bumps.get("payment").unwrap();

        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: user_token_account.to_account_info(),
                    to: receipient_token_account.to_account_info(),
                    authority: payment.to_account_info(),
                },
                &[&[
                    b"payment",
                    payment.authority.as_ref(),
                    payment.mint.as_ref(),
                    payment.receipient.as_ref(),
                    &[bump]]
                ]),
            payment.amount,
        )?;

        Ok(ThreadResponse::default())
    }

    pub fn handle_update_payment(ctx: Context<UpdatePayment>, amount: Option<u64>) -> Result<()> {
        msg!("Updating payment");

        let payment = &mut ctx.accounts.payment;
        if let Some(amount) = amount {
            payment.amount = amount;
        }

        Ok(())
    }
}