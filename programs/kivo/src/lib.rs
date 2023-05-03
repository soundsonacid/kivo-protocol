use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("8N3JeLHZP1uWVjZ6hwdC79MjTQWQ3gfmAQh4qTwc6GeF");

#[program]
pub mod kivo {
    use super::*;

    pub fn handle_initialize_user(ctx: Context<InitializeUser>, name: String) -> Result<()> {
        msg!("Initializing user");
        // Get mutable references to both the user account PDA and the owner (client-side user) pubkey
        let user_account = &mut ctx.accounts.user_account;
        let owner = &mut ctx.accounts.owner;

        if name.chars().count() > 16 {            // The maximum length of a username is 16 characters
            return Err(ErrorCode::NameTooLong.into())
        }

        user_account.name = name; 
        user_account.owner = owner.key();         // This should be the public key of the client-side user
        user_account.pubkey = user_account.key(); // This should be our User account PDA

        Ok(())
    }

    pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!("Handling deposit");
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
        msg!("Handling withdrawal");

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
        msg!("Making transfer");

        let seeds = &[
            b"user",
            ctx.accounts.sender_user_account.key.as_ref(),
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

    pub fn handle_create_transaction_account(ctx: Context<CreateTransactionAccount>, token: u16, amount: u64, time_stamp: u64) -> Result<()> {
        msg!("Creating request!");

        let user_transaction_account = &mut ctx.accounts.user_transaction_account;
        let receiver_transaction_account = &mut ctx.accounts.receiver_transaction_account;
        let user_account = &mut ctx.accounts.user_account;
        let receiver_account = &mut ctx.accounts.receiver_account;

        user_transaction_account.sender_pda = user_account.pubkey;
        user_transaction_account.token = token;
        user_transaction_account.amount = amount;
        user_transaction_account.time_stamp = time_stamp;
        user_transaction_account.receiver_transaction_pda = receiver_transaction_account.key();
        user_transaction_account.status = false;

        user_account.payments_sent = user_account.payments_sent.saturating_add(1);

        receiver_transaction_account.sender_pda = user_account.pubkey;
        receiver_transaction_account.token = token;
        receiver_transaction_account.amount = amount;
        receiver_transaction_account.time_stamp = time_stamp;
        receiver_transaction_account.receiver_transaction_pda = user_transaction_account.key();
        receiver_transaction_account.status = false;

        receiver_account.payments_received = receiver_account.payments_received.saturating_add(1);

        user_account.exit(&crate::id())?;
        user_transaction_account.exit(&crate::id())?;
        receiver_account.exit(&crate::id())?;
        receiver_transaction_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_edit_username(ctx: Context<EditUsername>, username: String) -> Result<()> {
        msg!("Changing username!");

        let user_account = &mut ctx.accounts.user_account;

        user_account.name = username;
        user_account.exit(&crate::id())?;
        
        Ok(())
    }
}

// PROGRAM ACCOUNTS & IMPLEMENTATIONS

#[account]
#[derive(Default)]
pub struct User {
    pub pubkey: Pubkey,         // This should be a PDA
    pub owner: Pubkey,          // This should be the public key of the client
    pub name: String,
    pub total_deposits: u64,
    pub total_withdraws: u64,
    pub payments_sent: u32,
    pub payments_received: u32,
}

#[account]
#[derive(Default)]
pub struct Transaction {
    pub sender_pda: Pubkey, // PDA 32
    pub token: u16, // TOKEN TYPE 4
    pub amount: u64, // AMOUNT 8
    pub time_stamp: u64, // TIME STAMP 8
    pub receiver_transaction_pda: Pubkey, // PDA 32
    pub status: bool // STATUS 1
}

impl User {
    pub fn increment_total_deposits(&mut self, amount: u64) -> Result<()> {       // Implement error check or remove Ok(()) result
        self.total_deposits = self.total_deposits.saturating_add(amount);

        Ok(())
    }

    pub fn increment_total_withdrawals(&mut self, amount: u64) -> Result <()> {
        self.total_withdraws = self.total_withdraws.saturating_add(amount);

        Ok(())
    }
}

// PROGRAM INSTRUCTIONS
// 1. InitializeUser
// 2. Deposit
// 3. Withdrawal
// 4. CreateTransactionAccount 
// 5. ExecuteTransaction
// 6. EditUsername

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 20 + 8 + 8, // disc + pk + pk + str + u64 + u64
        seeds = [b"user", owner.to_account_info().key.as_ref()], 
        bump,
    )]
    pub user_account: Account<'info, User>,  // This should be a PDA
    pub owner: Signer<'info>,                // This should be the public key of the client 
    #[account(mut)]
    pub payer: Signer<'info>,                // This should also be the public key of the client
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub depositor: UncheckedAccount<'info>,
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub pda_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdrawal<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub withdrawer: UncheckedAccount<'info>,
    #[account(mut)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,
     /// CHECK: This is not dangerous because we don't read or write from this account
    pub user_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub pda_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateTransactionAccount<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + 8 + 8 + 32 + 1, // pk + u16 + u64 + u64 + pk + bool
        seeds = [b"transaction",
                 user_account.to_account_info().key.as_ref(),
                 user_account.payments_sent.to_le_bytes().as_ref()],
        bump
    )]
    pub user_transaction_account: Account<'info, Transaction>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub receiver_account: Account<'info, User>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 4 + 8 + 8 + 32 + 1, // pk + u16 + u64 + u64 + pk + bool
        seeds = [b"transaction",
                receiver_account.to_account_info().key.as_ref(),
                receiver_account.payments_received.to_le_bytes().as_ref()],
        bump
    )]
    pub receiver_transaction_account: Account<'info, Transaction>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sender_user_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EditUsername<'info> {
    #[account(mut)]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>
}

#[error_code]
pub enum ErrorCode {
    #[msg("Username must be 16 characters or less!")]
    NameTooLong,
}