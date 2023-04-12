use anchor_lang::prelude::*;

declare_id!("8N3JeLHZP1uWVjZ6hwdC79MjTQWQ3gfmAQh4qTwc6GeF");

#[program]
pub mod kivo {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, name: String) -> Result<()> {
        // Get mutable references to both the user account pubkey and the owner (client-side user) pubkey
        let user_account = &mut ctx.accounts.user_account;
        let owner = &mut ctx.accounts.owner;

        if name.chars().count() > 16 { // The maximum length of a username is 16 characters
            return Err(ErrorCode::NameTooLong.into())
        }

        user_account.name = name;
        user_account.owner = owner.key(); // This should be the public key of the client-side user
        user_account.pubkey = user_account.key(); // This should be the public key of the User account

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 20 + 8 + 8 + 8 // disc + pk + pk + str + u64 + u64 + u64
    )]
    pub user_account: Account<'info, User>,
    pub owner: Signer<'info>, // This should be the public key of the client side user
    #[account(mut)]
    pub payer: Signer<'info>, // This should also be the public key of the client side user
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct User {
    pub pubkey: Pubkey, // This is the public key of the User account created by the Program
    pub owner: Pubkey, // This is the public key of the client-side user
    pub name: String,
    pub total_deposits: u64,
    pub total_withdraws: u64,
    pub available_deposits: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Username must be 16 characters or less!")]
    NameTooLong,
}

