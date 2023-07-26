use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::{
    token::*,
    associated_token::*,
};
use crate::state::lending_account::PassiveLendingAccount;

pub fn process(_ctx: Context<InitializeLendingVaults>) -> Result<()> {
    msg!("Initializing vaults");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLendingVaults<'info> {

  #[account(mut)]
  pub lending_account: Account<'info, PassiveLendingAccount>,

  #[account()]
  pub wsol_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = wsol_mint, 
            associated_token::authority = lending_account
            )]
  pub wsol_vault: Box<Account<'info, TokenAccount>>,

  #[account()]
  pub usdc_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = usdc_mint, 
            associated_token::authority = lending_account
            )]
  pub usdc_vault: Box<Account<'info, TokenAccount>>,  

  #[account()]
  pub usdt_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = usdt_mint, 
            associated_token::authority = lending_account
            )]
  pub usdt_vault: Box<Account<'info, TokenAccount>>,

  #[account()]
  pub uxd_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = uxd_mint, 
            associated_token::authority = lending_account
            )]
  pub uxd_vault: Box<Account<'info, TokenAccount>>,

  #[account()]
  pub bonk_mint: Box<Account<'info, Mint>>,

  #[account(init, 
            payer = payer, 
            associated_token::mint = bonk_mint, 
            associated_token::authority = lending_account
            )]
  pub bonk_vault: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(address = anchor_spl::token::ID)]
  pub token_program: Program<'info, Token>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  #[account(address = system_program::ID)]
  pub system_program: Program<'info, System>
}