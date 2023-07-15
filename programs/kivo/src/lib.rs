use anchor_lang::prelude::*;
use instructions::*;

pub mod state;
pub mod error;
pub mod constants;
mod instructions;

declare_id!("7bRUosmoUkYVgZJHj2UDWM6kgHoy748R6NGweiDEk2vZ");

#[program]
pub mod kivo {
    use super::*;

    // User endpoints
    // 1. handle_initialize_user
    // 2. handle_initialize_user_vaults
    // 3. handle_deposit
    // 4. handle_withdrawal
    // 5. handle_unwrap_withdrawal
    // 6. handle_edit_username
    // 7. handle_add_friend
    // 8. handle_set_preferred_token
    // 9. handle_disable_preferred_token
    
    pub fn handle_initialize_user(
            ctx: Context<InitializeUser>, 
            name: [u8; 16], 
            account_type: u8
        ) -> Result<()> {
        user_init::process(ctx, name, account_type)
    }

    pub fn handle_initialize_user_vaults(
            ctx: Context<InitializeUserVaults>
        ) -> Result<()> {
        user_vaults_init::process(ctx)
    }

    pub fn handle_deposit(
            ctx: Context<Deposit>, 
            amount: u64
        ) -> Result<()> {
        user_deposit::process(ctx, amount)
    }

    pub fn handle_withdrawal(
            ctx: Context<Withdrawal>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
        user_withdraw::process(ctx, amount, bump)
    }
    
    pub fn handle_unwrap_withdrawal(
            ctx: Context<UnwrapWithdrawal>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
        user_unwrap_withdraw::process(ctx, amount, bump)
    }

    pub fn handle_edit_username(
        ctx: Context<EditUsername>, 
        name: [u8; 16]
    ) -> Result<()> {
        username_edit::process(ctx, name)
    }

    pub fn handle_add_friend(
        ctx: Context<AddFriend>
    ) -> Result<()> {
        user_add_friend::process(ctx)
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
    // 1. handle_execute_transaction
    // 2. handle_create_request
    // 3. handle_fulfill_request

    pub fn handle_execute_transaction(
            ctx: Context<ExecuteTransaction>, 
            amount: u64, 
            bump: u8, 
            time_stamp: u64
        ) -> Result<()> {
        transaction_execute::process(ctx, amount, bump, time_stamp)
    }

    pub fn handle_create_request(
            ctx: Context<CreateRequest>, 
            amount: u64, 
            time_stamp: u64
        ) -> Result<()> {
        transaction_request_create::process(ctx, amount, time_stamp)
    }

    pub fn handle_fulfill_request(
            ctx: Context<FulfillRequest>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
        transaction_request_fufill::process(ctx, amount, bump)
    }

    // Contract endpoints
    // 1. handle_propose_contract
    // 2. handle_accept_contract
    // 3. handle_reject_contract
    // 4. handle_settle_contract_payment (only called by contract threads)

    pub fn handle_propose_contract(
            ctx: Context<ProposeContract>, 
            amount: u64, 
            schedule: String, 
            id: String, 
            bump: u8, 
            num_payments_obligated: u64
        ) -> Result<()> {
        contract_propose::process(ctx, amount, schedule, id, bump, num_payments_obligated)
    }

    pub fn handle_accept_contract(
            ctx: Context<AcceptContract>, 
            obligor_bump: u8, 
            user_bump: u8
        ) -> Result<()> {
        contract_accept::process(ctx, obligor_bump, user_bump)
    }

    pub fn handle_reject_contract(
            ctx: Context<RejectContract>
        ) -> Result<()> {
        contract_reject::process(ctx)
    }

    pub fn handle_settle_contract_payment(
            ctx: Context<SettleContractPayment>
        ) -> Result<clockwork_sdk::state::ThreadResponse> {
        contract_settle::process(ctx)
    }
}