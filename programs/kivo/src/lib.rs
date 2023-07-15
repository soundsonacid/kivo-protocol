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

    pub fn handle_initialize_user(
            ctx: Context<InitializeUser>, 
            name: [u8; 16], 
            account_type: u8
        ) -> Result<()> {
            initialize_user::process(ctx, name, account_type)
    }

    pub fn handle_initialize_user_vaults(
            ctx: Context<InitializeUserVaults>
        ) -> Result<()> {
            initialize_vaults::process(ctx)
    }

    pub fn handle_deposit(
            ctx: Context<Deposit>, 
            amount: u64
        ) -> Result<()> {
            deposit::process(ctx, amount)
    }

    pub fn handle_withdrawal(
            ctx: Context<Withdrawal>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
            withdrawal::process(ctx, amount, bump)
    }
    
    pub fn handle_unwrap_withdrawal(
            ctx: Context<UnwrapWithdrawal>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
            unwrap_withdrawal::process(ctx, amount, bump)
    }
    
    pub fn handle_execute_transaction(
            ctx: Context<ExecuteTransaction>, 
            amount: u64, 
            bump: u8, 
            time_stamp: u64
        ) -> Result<()> {
            execute_transaction::process(ctx, amount, bump, time_stamp)
    }

    pub fn handle_create_request(
            ctx: Context<CreateRequest>, 
            amount: u64, 
            time_stamp: u64
        ) -> Result<()> {
            create_request::process(ctx, amount, time_stamp)
    }

    pub fn handle_fulfill_request(
            ctx: Context<FulfillRequest>, 
            amount: u64, 
            bump: u8
        ) -> Result<()> {
            fulfill_request::process(ctx, amount, bump)
    }

    pub fn handle_edit_username(
            ctx: Context<EditUsername>, 
            name: [u8; 16]
        ) -> Result<()> {
            edit_username::process(ctx, name)
    }

    pub fn handle_set_preferred_token(
            ctx: Context<SetPreferredToken>
        ) -> Result<()> {
            set_preferred_token::process(ctx)
    }

    pub fn handle_disable_preferred_token(
            ctx: Context<DisablePreferredToken>
        ) -> Result<()> {
            disable_preferred_token::process(ctx)
    }

    pub fn handle_add_friend(
            ctx: Context<AddFriend>
        ) -> Result<()> {
            add_friend::process(ctx)
    }

    pub fn handle_propose_contract(
            ctx: Context<ProposeContract>, 
            amount: u64, 
            schedule: String, 
            id: String, 
            bump: u8, 
            num_payments_obligated: u64
        ) -> Result<()> {
            propose_contract::process(ctx, amount, schedule, id, bump, num_payments_obligated)
    }

    pub fn handle_accept_contract(
            ctx: Context<AcceptContract>, 
            obligor_bump: u8, 
            user_bump: u8
        ) -> Result<()> {
            accept_contract::process(ctx, obligor_bump, user_bump)
    }

    pub fn handle_reject_contract(
            ctx: Context<RejectContract>
        ) -> Result<()> {
            reject_contract::process(ctx)
    }

    pub fn handle_settle_contract_payment(
            ctx: Context<SettleContractPayment>
        ) -> Result<clockwork_sdk::state::ThreadResponse> {
            settle_contract_payment::process(ctx)
    }
}