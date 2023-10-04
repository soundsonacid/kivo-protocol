use anchor_lang::{
    prelude::*,
    solana_program::system_program
};
use anchor_spl::{
    token::*,
    associated_token::*,
};

pub fn process(ctx: Context<InitGroupVaults>) -> Result<()> {
    msg!("Initializing vaults for group {}", ctx.accounts.group.key().to_string());

    Ok(())
}

#[derive(Accounts)]
pub struct InitGroupVaults<'info> {
    #[account(
        init, 
        payer = payer, 
        associated_token::mint = wsol_mint, 
        associated_token::authority = group
    )]
    pub wsol_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init, 
        payer = payer, 
        associated_token::mint = usdc_mint, 
        associated_token::authority = group
    )]
    pub usdc_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init, 
        payer = payer, 
        associated_token::mint = usdt_mint, 
        associated_token::authority = group
    )]
    pub usdt_vault: Box<Account<'info, TokenAccount>>,
    
    #[account(
        init, 
        payer = payer, 
        associated_token::mint = uxd_mint, 
        associated_token::authority = group
    )]
    pub uxd_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init, 
        payer = payer, 
        associated_token::mint = bonk_mint, 
        associated_token::authority = group
    )]
    pub bonk_vault: Box<Account<'info, TokenAccount>>,

    pub wsol_mint: Box<Account<'info, Mint>>,

    pub usdc_mint: Box<Account<'info, Mint>>,

    pub usdt_mint: Box<Account<'info, Mint>>,

    pub uxd_mint: Box<Account<'info, Mint>>,

    pub bonk_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub group: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}