use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::{
    token::*,
    associated_token::*,
};
use crate::state::user::User;

pub fn process(_ctx: Context<InitializeUserVaults>) -> Result<()> {
    msg!("Initializing vaults");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUserVaults<'info> {

  #[account(init_if_needed,
            payer = payer, 
            space = 8 + std::mem::size_of::<User>(), 
            seeds = [
                b"user", 
                payer.key.as_ref()
                ], 
            bump            
            )]
  pub user_account: Account<'info, User>,

  #[account()]
  pub wsol_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = wsol_mint, 
            associated_token::authority = user_account
            )]
  pub wsol_vault: Box<Account<'info, TokenAccount>>,

  #[account()]
  pub usdc_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = usdc_mint, 
            associated_token::authority = user_account
            )]
  pub usdc_vault: Box<Account<'info, TokenAccount>>,  

  #[account()]
  pub usdt_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = usdt_mint, 
            associated_token::authority = user_account
            )]
  pub usdt_vault: Box<Account<'info, TokenAccount>>,

  #[account()]
  pub uxd_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = uxd_mint, 
            associated_token::authority = user_account
            )]
  pub uxd_vault: Box<Account<'info, TokenAccount>>,

  #[account()]
  pub bonk_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = bonk_mint, 
            associated_token::authority = user_account
            )]
  pub bonk_vault: Box<Account<'info, TokenAccount>>,

  #[account()]
  pub lst_mint: Box<Account<'info, Mint>>,

  #[account(init,
            payer = payer,
            associated_token::mint = lst_mint,
            associated_token::authority = user_account
            )]
  pub lst_vault: Box<Account<'info, TokenAccount>>,
  
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(address = anchor_spl::token::ID)]
  pub token_program: Program<'info, Token>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  #[account(address = system_program::ID)]
  pub system_program: Program<'info, System>
}