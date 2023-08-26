// Created by Frank
use anchor_lang::prelude::*;
use instructions::*;

pub mod state;
pub mod error;
pub mod constants;
mod instructions;

declare_id!("5p53sbjVpoDizJ8CaaVrJ7ZirXAH6AZNUysd75rnn98p");

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
        ) -> Result<()> {
        user_withdraw::process(ctx, amount)
    }
    
    pub fn handle_unwrap_withdrawal(
            ctx: Context<UnwrapWithdrawal>, 
            amount: u64, 
        ) -> Result<()> {
        user_unwrap_withdraw::process(ctx, amount)
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
    // 4. handle_reject_request

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

    // Contract endpoints
    // 1. handle_propose_contract
    // 2. handle_accept_contract
    // 3. handle_reject_contract
    // 4. handle_settle_contract_payment (only called by contract threads)

    pub fn handle_propose_contract(
            ctx: Context<ProposeContract>, 
            amount: u64, 
            num_payments_obligated: u32
        ) -> Result<()> {
        contract_propose::process(ctx, amount, num_payments_obligated)
    }

    pub fn handle_accept_contract(
            ctx: Context<AcceptContract>, 
            schedule: String
        ) -> Result<()> {
        contract_accept::process(ctx, schedule)
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

    // Lending endpoints
    // 1. handle_lending_deposit (used for depositing & repaying borrows)

    pub fn handle_lending_deposit(
            ctx: Context<LendingDeposit>,
            amount: u64
    ) -> Result<()> {
        lending_deposit::process(ctx, amount)
    }

    // Group endpoints
    // 1. handle_group_create
    // 2. handle_group_invite
    // 3. handle_group_join
    // 4. handle_group_leave
    // 5. handle_group_kick (Group Admins only)
    // 6. handle_group_transfer (Group Admins only)

    pub fn handle_group_create(
            ctx: Context<CreateGroup>,
            group_id: u32,
            group_name: [u8; 32],
    ) -> Result<()> {
        group_create::process(ctx, group_id, group_name)
    }

    pub fn handle_group_invite(
            ctx: Context<GroupInvite>
    ) -> Result<()> {
        group_invite::process(ctx)
    }

    pub fn handle_group_join(
            ctx: Context<GroupJoin>
    ) -> Result<()> { 
        group_join::process(ctx)
    }

    pub fn handle_group_leave(
            ctx: Context<LeaveGroup>
    ) -> Result<()> {
        group_leave::process(ctx)
    }

    pub fn handle_group_kick(
            ctx: Context<KickMemberFromGroup>
    ) -> Result<()> {
        group_kick::process(ctx)
    }

    pub fn handle_group_transfer(
            ctx: Context<TransferGroupOwnership>
    ) -> Result<()> {
        group_transfer::process(ctx)
    }

    // Paid Group endpoints
    // 1. handle_pgroup_create
    // 2. handle_pgroup_invite
    // 3. handle_pgroup_join
    // 4. handle_pgroup_leave
    // 5. handle_pgroup_kick (Group Admins only)
    // 6. handle_pgroup_transfer (Group Admins only)
    // 7. handle_pgroup_change_fee (Group Admins only)
    // 8. handle_pgroup_makepayment (Called internally by recurring payment Group threads only)

    pub fn handle_pgroup_create(
            ctx: Context<CreatePaidGroup>,
            group_id: u32,
            group_name: [u8; 32],
            fee: u64,
            recurring: bool
    ) -> Result<()> {
        paidgroup_create::process(ctx, group_id, group_name, fee, recurring)
    }

    pub fn handle_pgroup_invite(
            ctx: Context<PaidGroupInvite>
    ) -> Result<()> {
        paidgroup_invite::process(ctx)
    }

    pub fn handle_pgroup_join(
            ctx: Context<PaidGroupJoin>,
            schedule: Option<String>,
    ) -> Result<()> {
        paidgroup_join::process(ctx, schedule)
    }

    pub fn handle_pgroup_makepayment(
        ctx: Context<PaidGroupMakePayment>
    ) -> Result<clockwork_sdk::state::ThreadResponse> {
        paidgroup_makepayment::process(ctx)
    }
}