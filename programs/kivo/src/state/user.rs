use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct User {
    pub owner: Pubkey, // OWNER PUBLIC KEY 32
    pub username: String, // USERNAME 20
    pub account_type: u8, // ACCOUNT TYPE 1
    pub total_deposits: u64, // DEPOSITS 8
    pub total_withdraws: u64, // WITHDRAWS 8
    pub payments_sent: u32, // SENT 4
    pub payments_received: u32, // RECEIVED 4
    pub num_friends: u32,
    pub num_contracts: u32,
}

impl User {
    pub(crate) fn set_owner(&mut self, owner: Pubkey) {
        self.owner = owner;
    }
    
    pub(crate) fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub(crate) fn set_account_type(&mut self, account_type: u8) {
        self.account_type = account_type;
    }

    pub(crate) fn increment_payments_sent(&mut self) {
        self.payments_sent = self.payments_sent.saturating_add(1);
    }

    pub(crate) fn increment_payments_received(&mut self) {
        self.payments_received = self.payments_received.saturating_add(1);
    }

    pub(crate) fn increment_friends(&mut self) {
        self.num_friends = self.num_friends.saturating_add(1);
    }

    pub(crate) fn increment_contracts(&mut self) {
        self.num_contracts = self.num_contracts.saturating_add(1);
    }

    // pub(crate) fn increment_total_deposits(&mut self, amount: u64) {  
    //     self.total_deposits = self.total_deposits.saturating_add(amount);
    // }

    // pub(crate) fn increment_total_withdrawals(&mut self, amount: u64)  {
    //     self.total_withdraws = self.total_withdraws.saturating_add(amount);
    // }
}

#[account]
pub struct Username {
    pub user_account: Pubkey, // 32
    pub username: String // 20
}

impl Username {
    pub(crate) fn set_owner(&mut self, user_account: Pubkey) {
        self.user_account = user_account;
    }
    
    pub(crate) fn set_username(&mut self, username: String) {
        self.username = username;
    }
}

#[account]
pub struct Friend {
    pub user_account: Pubkey,
    pub friend_account: Pubkey,
    pub friend_number: u32,
}

impl Friend {
    pub(crate) fn set_user_account(&mut self, user_account: Pubkey) {
        self.user_account = user_account;
    }

    pub(crate) fn set_friend_account(&mut self, friend_account: Pubkey) {
        self.friend_account = friend_account;
    }

    pub(crate) fn set_friend_number(&mut self, friend_number: u32) {
        self.friend_number = friend_number;
    }
}