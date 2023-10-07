// Created by Frank for Kiwi Group
use anchor_lang::prelude::*;
use instructions::*;

pub mod state;
pub mod error;
pub mod constants;
mod instructions;

declare_id!("AgcadSiiADx1LoR4fkwTJWQehg5esUpGSuEDLrKrhLT1");

#[program]
pub mod kivo {

    use super::*;

    // User endpoints
    pub fn handle_initialize_user(
            ctx: Context<InitializeUser>, 
            name: [u8; 16], 
        ) -> Result<()> {
        user_init::process(ctx, name)
    }

    pub fn handle_initialize_user_vaults(
            ctx: Context<InitializeUserVaults>
        ) -> Result<()> {
        user_vaults_init::process(ctx)
    }

    pub fn handle_withdrawal(
            ctx: Context<Withdrawal>, 
            amount: u64, 
            withdraw_all: Option<bool>,
        ) -> Result<()> {
        user_withdraw::process(ctx, amount, withdraw_all)
    }
    
    pub fn handle_unwrap_withdrawal(
            ctx: Context<UnwrapWithdrawal>, 
            amount: u64, 
            withdraw_all: Option<bool>,
        ) -> Result<()> {
        user_unwrap_withdraw::process(ctx, amount, withdraw_all)
    }

    pub fn handle_set_preferred_token(
            ctx: Context<SetPreferredToken>
        ) -> Result<()> {
        user_preferred_token_set::process(ctx)
    }

    pub fn handle_disable_preferred_token(
            ctx: Context<DisablePreferredToken>
        ) -> Result<()> {
        user_preferred_token_disable::process(ctx)
    }
    
    // Transaction endpoints
    pub fn handle_execute_transaction(
            ctx: Context<ExecuteTransaction>, 
            amount: u64, 
        ) -> Result<()> {
        transaction_execute::process(ctx, amount)
    }
    
    pub fn handle_create_request(
            ctx: Context<CreateRequest>, 
            amount: u64, 
        ) -> Result<()> {
        transaction_request_create::process(ctx, amount)
    }

    pub fn handle_fulfill_request(
            ctx: Context<FulfillRequest>, 
            amount: u64, 
        ) -> Result<()> {
        transaction_request_fufill::process(ctx, amount)
    }

    pub fn handle_reject_request(
            ctx: Context<RejectRequest>
    ) -> Result<()> {
        transaction_request_reject::process(ctx)
    }

    pub fn handle_preferred_tx_exec(
            ctx: Context<PreferredSwapExec>,
            amt: u64,
            data: Vec<u8>,
    ) -> Result<()> {
        preferred_tx_exec::process(ctx, amt, data)
    }

    pub fn handle_preferred_tx_fulfill(
            ctx: Context<PreferredSwapFulfill>,
            amt: u64,
            data: Vec<u8>,
    ) -> Result<()> {
        preferred_tx_fulfill::process(ctx, amt, data)
    }

    // Lending endpoints
    pub fn handle_lending_deposit(
            ctx: Context<LendingDeposit>,
            amount: u64
    ) -> Result<()> {
        lending_deposit::process(ctx, amount)
    }

    // Group endpoints
    pub fn handle_group_create(
            ctx: Context<CreateGroup>,
    ) -> Result<()> {
        group_create::process(ctx)
    }

    pub fn handle_group_vaults_init(
            ctx: Context<InitGroupVaults>
    ) -> Result<()> {
        group_vaults_init::process(ctx)
    }

    pub fn handle_group_deposit(
            ctx: Context<DepositToGroupWallet>,
            amount: u64,
    ) -> Result<()> {
        group_deposit::process(ctx, amount)
    }

    pub fn handle_group_deposit_signed(
            ctx: Context<GroupDepositSigned>,
            amount: u64
    ) -> Result<()> {
        group_deposit_signed::process(ctx, amount)
    }

    pub fn handle_group_withdrawal(
            ctx: Context<WithdrawFromGroupWallet>,
            amount: u64,
            withdraw_all: Option<bool>
    ) -> Result<()> {
        group_withdraw::process(ctx, amount, withdraw_all)
    }

    pub fn handle_ape(
            ctx: Context<Ape>,
            amount: u64,
            data: Vec<u8>
    ) -> Result<()> {
        ape::process(ctx, amount, data)
    }

    pub fn handle_split(
            ctx: Context<Split>,
            amount: u64,
    ) -> Result<()> {
        split::process(ctx, amount)
    }

    pub fn handle_swap_split(
            ctx: Context<SwapSplit>,
            amount: u64,
            data: Vec<u8>,
    ) -> Result<()> {
        swap_split::process(ctx, amount, data)
    }
}