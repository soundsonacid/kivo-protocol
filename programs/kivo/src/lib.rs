use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, TokenAccount, Token, spl_token::instruction::AuthorityType, SetAuthority,
};

declare_id!("8N3JeLHZP1uWVjZ6hwdC79MjTQWQ3gfmAQh4qTwc6GeF");

// EGMxcpUUReyH3wQ4zLkHtEctcdtn8A3RJw3hY9tJTrwJ
// Don't get rid of this.  It's my devnet SOL address.

#[program]
pub mod kivo {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, name: String) -> Result<()> {
        // Get mutable references to both the user account pubkey and the owner (client-side user) pubkey
        // let (user_account, _) = Pubkey::find_program_address(&[b"user", name.as_bytes()], ctx.program_id);
        let user_account = &mut ctx.accounts.user_account;
        let owner = &mut ctx.accounts.owner;

        // Add check if username is unique
        if name.chars().count() > 16 {            // The maximum length of a username is 16 characters
            return Err(ErrorCode::NameTooLong.into())
        }

        user_account.name = name;
        user_account.owner = owner.key();         // This should be the public key of the client-side user
        user_account.pubkey = user_account.key(); // This should be the public key of the User account

        Ok(())
    }

    // pub fn initialize_vault(ctx: Context<InitializeVault>, authority: Pubkey) -> Result<()> {
    //     // Transfer authority of the vault account to our vault authority PDA, provided in fn signature.
    //     token::set_authority(
    //         ctx.accounts.get_vault_cpi_context(),
    //         AuthorityType::AccountOwner,
    //         Some(authority),
    //     )?;
        
    //     Ok(())
    // }

    // pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    //     handle_deposit(ctx, amount)
    // }

    pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Add check for if deposit is 0
        // Add check for if user is bankrupt
        // Add USD calculation via oracle
        let user_account = &mut ctx.accounts.user_account;
        user_account.increment_deposits(amount)?; 
        user_account.exit(&crate::id())?;   // Persist account data mutation
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct User {
    pub pubkey: Pubkey,         // This is the public key of the User account created by the Program
    pub owner: Pubkey,          // This is the public key of the client-side user
    pub name: String,
    pub total_deposits: u64,
    pub total_withdraws: u64,
    pub available_deposits: u64,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 20 + 8 + 8 + 8, // disc + pk + pk + str + u64 + u64 + u64
        seeds = [b"user", name.as_bytes()], 
        bump,
    )]
    pub user_account: Account<'info, User>,  // This should be a PDA
    pub owner: Signer<'info>,                // This should be the public key of the client side user
    #[account(mut)]
    pub payer: Signer<'info>,                // This should also be the public key of the client side user
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl User {
    pub fn increment_deposits(&mut self, amount: u64) -> Result<()> {       // Implement error check or remove Ok(()) result
        self.total_deposits = self.total_deposits.saturating_add(amount);
        self.available_deposits = self.available_deposits.saturating_add(amount);

        Ok(())
    }

    pub fn increment_withdrawals(&mut self, amount: u64) {
        self.total_withdraws = self.total_withdraws.saturating_add(amount);
    }

    pub fn decrement_deposits(&mut self, amount: u64) {
        self.total_deposits = self.total_deposits.saturating_sub(amount);
    }
}


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_account: Account<'info, User>,
    // pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// #[derive(Accounts)]
// pub struct InitializeVault<'info> {
//     #[account(
//         init,
//         payer = payer,
//         token::mint = mint,
//         token::authority = payer,
//     )]
//     pub vault: Account<'info, TokenAccount>, // This is the public key of our new Token Account vault
//     #[account(mut)]
//     pub payer: Signer<'info>,                // This should be a PDA
//     pub mint: Account<'info, Mint>,          // Mint address of Token Account to be created
//     pub rent: Sysvar<'info, Rent>,
//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }

// // Creates a new CpiContext with our SetAuthority ix to change the Token Account authority to our PDA
// impl<'info> InitializeVault<'info> { 
//     fn get_vault_cpi_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
//         let accounts = SetAuthority {
//             account_or_mint: self.vault.to_account_info(), // account, not mint
//             current_authority: self.payer.to_account_info(),
//         };
//         CpiContext::new(self.token_program.to_account_info(), accounts)
//     }
// }

#[error_code]
pub enum ErrorCode {
    #[msg("Username must be 16 characters or less!")]
    NameTooLong,
}

