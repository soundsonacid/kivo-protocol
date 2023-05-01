use anchor_lang::prelude::*;
use anchor_spl::token::*;
// use anchor_lang::solana_program::system_instruction;

declare_id!("8N3JeLHZP1uWVjZ6hwdC79MjTQWQ3gfmAQh4qTwc6GeF");

// EGMxcpUUReyH3wQ4zLkHtEctcdtn8A3RJw3hY9tJTrwJ
// Don't get rid of this.  It's my devnet SOL address.

#[program]
pub mod kivo {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, name: String) -> Result<()> {
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

    pub fn handle_withdrawal(ctx: Context<Withdrawal>, amount: u64) -> Result<()> {
        msg!("Handling withdrawal");
        // Add check for if amount is greater than available deposits 
        // let user_account = &mut ctx.accounts.user_account;

        let cpi_accounts = Transfer {
            from: ctx.accounts.pda_token_account.to_account_info(),
            to: ctx.accounts.withdrawer_token_account.to_account_info(),
            authority: ctx.accounts.user_account.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_context, amount)?;

        // user_account.increment_total_withdrawals(amount)?;
        // user_account.exit(&crate::id())?;

        Ok(())
    }

    pub fn handle_transfer(ctx: Context<Send>, amount: u64) -> Result<()> {
        msg!("Making transfer");

        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: ctx.accounts.sender_user_account.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_context, amount)?;

        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct User {
    pub pubkey: Pubkey,         // This should be a PDA
    pub owner: Pubkey,          // This should be the public key of the client
    pub name: String,
    pub total_deposits: u64,
    pub total_withdraws: u64,
}

#[derive(Accounts)]
#[instruction(name: String)]
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

impl User {
    pub fn increment_deposits(&mut self, amount: u64) -> Result<()> {       // Implement error check or remove Ok(()) result
        self.total_deposits = self.total_deposits.saturating_add(amount);

        Ok(())
    }

    pub fn increment_total_withdrawals(&mut self, amount: u64) -> Result <()> {
        self.total_withdraws = self.total_withdraws.saturating_add(amount);

        Ok(())
    }
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
pub struct Send<'info> {
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

#[error_code]
pub enum ErrorCode {
    #[msg("Username must be 16 characters or less!")]
    NameTooLong,
}