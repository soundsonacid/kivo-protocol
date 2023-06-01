use anchor_lang::prelude::*;

#[account]
pub struct Transaction {
    pub sender_account: Pubkey, // PDA 32
    pub mint: Pubkey, // MINT 32
    pub amount: u64, // AMOUNT 8
    pub time_stamp: u64, // TIME STAMP 8
    pub receiver_transaction_account: Pubkey, // PDA 32
    pub status: bool // STATUS 1
}

impl Transaction {
    pub(crate) fn set_sender_account(&mut self, sender_account: Pubkey) {
        self.sender_account = sender_account;
    }
    
    pub(crate) fn set_token(&mut self, token: Pubkey) {
        self.mint = token;
    }

    pub(crate) fn set_amount(&mut self, amount: u64) {
        self.amount = amount;
    }

    pub(crate) fn set_time_stamp(&mut self, time_stamp: u64) {
        self.time_stamp = time_stamp;
    }

    pub(crate) fn set_receiver_transaction_account(&mut self, receiver_transaction_account: Pubkey) {
        self.receiver_transaction_account = receiver_transaction_account;
    }

    pub(crate) fn set_status(&mut self, status: bool) {
        self.status = status;
    }
}