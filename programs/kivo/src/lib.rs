use anchor_lang::prelude::*;
use anchor_spl::token::*;
// use jupiter_cpi;
use clockwork_sdk::state::ThreadResponse;

use crate::instructions::user::*;
use crate::instructions::transaction::*;
use crate::instructions::contract::*;
use crate::state::contract::*;

pub mod state;
pub mod instructions;

declare_id!("HyA8SiVhkkYoidUuFkmVXWDgRtiiwQTy465GwH5m6XSw");

#[program]
pub mod kivo {
    use super::*;

    pub fn handle_initialize_user(ctx: Context<InitializeUser>, name: String, account_type: u8) -> Result<()> {
        msg!("Initalizing user!");

        if name.chars().count() > 16 {
            return Err(KivoErrorCode::NameTooLong.into());
        }

        let user_account = &mut ctx.accounts.user_account;
        let username_account = &mut ctx.accounts.username_account;

        username_account.set_username(name.clone());

        user_account.set_owner(ctx.accounts.owner.clone().key());
        user_account.set_username(name);
        user_account.set_account_type(account_type);
        
        username_account.exit(&crate::id())?;
        user_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!("Depositing");
        // Add check for if amount is 0
        // Add check for if user is bankrupt / in liquidation
        // Add USD calculation via oracle
        let cpi_accounts = Transfer {
            from: ctx.accounts.depositor_token_account.to_account_info(),
            to: ctx.accounts.pda_token_account.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        // user_account.increment_deposits(amount)?; 
        // user_account.exit(&crate::id())?;   // Persist account data mutation

        Ok(())
    }

    pub fn handle_withdrawal(ctx: Context<Withdrawal>, amount: u64, bump: u8) -> Result<()> {
        msg!("Withdrawing");

        let seeds = &[
            b"user",
            ctx.accounts.withdrawer.key.as_ref(),
            &[bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.pda_token_account.to_account_info(),
            to: ctx.accounts.withdrawer_token_account.to_account_info(),
            authority: ctx.accounts.user_account.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, amount)?;

        // user_account.increment_total_withdrawals(amount)?;
        // user_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_execute_transaction(ctx: Context<ExecuteTransaction>, amount: u64, bump: u8) -> Result<()> {
        msg!("Executing transaction");

        let seeds = &[
            b"user",
            ctx.accounts.sender.key.as_ref(),
            &[bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: ctx.accounts.sender_user_account.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, amount)?;

        Ok(())
    }

    // pub fn handle_execute_swap_transaction(ctx: Context<ExecuteSwapTransaction>, amount: u64, bump: u8) -> Result<()> {
    //     msg!("Executing swap transaction");

    //     let seeds = &[
    //         b"user",
    //         ctx.accounts.sender.key.as_ref(),
    //         &[bump]
    //     ];

    //     let signer_seeds = &[&seeds[..]];

    //     let swap_cpi_context = ctx.accounts.get_swap_cpi_context(signer_seeds);

    //     jupiter_cpi::cpi::token_swap(swap_cpi_context)?;

    //     msg!("Swap complete");
    //     msg!("Executing transaction");

    //     let cpi_accounts = Transfer {
    //         from: ctx.accounts.sender_destination_token_account.to_account_info(),
    //         to: ctx.accounts.receiver_token_account.to_account_info(),
    //         authority: ctx.accounts.sender_user_account.to_account_info(),
    //     };

    //     let cpi_program = ctx.accounts.token_program.to_account_info().clone();

    //     let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    //     transfer(cpi_context, amount)?;

    //     Ok(())
    // }

    pub fn handle_create_transaction_account(ctx: Context<CreateTransactionAccount>, 
                                            amount: u64, 
                                            time_stamp: u64) -> Result<()> {
        msg!("Creating transaction account");

        let user_transaction_account = &mut ctx.accounts.user_transaction_account;
        let receiver_transaction_account = &mut ctx.accounts.receiver_transaction_account;
        let user_account = &mut ctx.accounts.user_account;
        let receiver_account = &mut ctx.accounts.receiver_account;
        let token = &mut ctx.accounts.token;
        
        user_transaction_account.set_sender_account(user_account.key());
        user_transaction_account.set_token(token.key());
        user_transaction_account.set_amount(amount);
        user_transaction_account.set_time_stamp(time_stamp);
        user_transaction_account.set_receiver_transaction_account(receiver_transaction_account.key());
        user_transaction_account.set_status(false);

        user_account.increment_payments_sent();

        receiver_transaction_account.set_sender_account(user_transaction_account.sender_account);
        receiver_transaction_account.set_token(token.key());
        receiver_transaction_account.set_amount(amount);
        receiver_transaction_account.set_time_stamp(time_stamp);
        receiver_transaction_account.set_receiver_transaction_account(user_transaction_account.key());
        receiver_transaction_account.set_status(false);

        receiver_account.increment_payments_received();

        user_account.exit(&crate::id())?;
        user_transaction_account.exit(&crate::id())?;
        receiver_account.exit(&crate::id())?;
        receiver_transaction_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_edit_username(ctx: Context<EditUsername>, username: String) -> Result<()> {
        msg!("Editing username");

        let new_username_account = &mut ctx.accounts.new_username_account;
        let user_account = &mut ctx.accounts.user_account;
        
        ctx.accounts.old_username_account.close(user_account.to_account_info())?;

        new_username_account.set_owner(user_account.key());
        new_username_account.set_username(username.clone());

        user_account.set_username(username);

        user_account.exit(&crate::id())?;
        new_username_account.exit(&crate::id())?;
        
        Ok(())
    }

    pub fn handle_add_friend(ctx: Context<AddFriend>) -> Result<()> {
        msg!("Adding friend");

        let user_account = &mut ctx.accounts.user_account;
        let friend_account = &mut ctx.accounts.friend_account;
        let friend = &mut ctx.accounts.new_friend;

        friend.set_user_account(user_account.key());
        friend.set_friend_account(friend_account.key());
        friend.set_friend_number(user_account.num_friends);

        user_account.increment_friends();
        user_account.exit(&crate::id())?;
        Ok(())
    }

    pub fn handle_create_payment(ctx: Context<CreatePayment>, amount: u64, bump: u8) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let user_token_account = &mut ctx.accounts.user_token_account;
        let mint = &ctx.accounts.mint;
        let payment = &mut ctx.accounts.payment;
        let receipient = &ctx.accounts.receipient;
        let token_program = &ctx.accounts.token_program;

        payment.new(
            amount,
            user_account.key(),
            mint.key(),
            receipient.key(),
        )?;

        approve(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Approve {
                    authority: user_account.to_account_info(),
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

        user_account.increment_contracts();
        user_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_disburse_payment(ctx: Context<DisbursePayment>) -> Result<ThreadResponse> {
        let authority_token_account = &mut ctx.accounts.authority_token_account;
        let payment = &mut ctx.accounts.payment;
        let receipient_token_account = &ctx.accounts.receipient;
        let token_program = &ctx.accounts.token_program;

        let bump = *ctx.bumps.get("payment").unwrap();

        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: authority_token_account.to_account_info(),
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
        let payment = &mut ctx.accounts.payment;
        if let Some(amount) = amount {
            payment.amount = amount;
        }

        Ok(())
    }
}


#[error_code]
pub enum KivoErrorCode {
    #[msg("Username must be 16 characters or less!")]
    NameTooLong,
}