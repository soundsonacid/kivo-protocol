use anchor_lang::prelude::*;
use crate::state::traits::{ Size, TransactionAccount };

#[account]
pub struct Transaction {
    pub sender_account: Pubkey, // PDA 32
    pub sender_username: [u8; 16],
    pub mint: Pubkey, // MINT 32
    pub amount: u64, // AMOUNT 8
    pub time_stamp: u64, // TIME STAMP 8
    pub receiver_account: Pubkey,
    pub receiver_username: [u8; 16],
    pub receiver_transaction_account: Pubkey, // PDA 32
    pub status: bool // STATUS 1
}

impl Size for Transaction {
    const SIZE: usize = 177;
}

impl TransactionAccount for Account<'_, Transaction> {
    fn new(
        &mut self,
        sender_account: Pubkey,
        sender_username: [u8; 16],
        mint: Pubkey,
        amount: u64,
        time_stamp: u64,
        receiver_account: Pubkey,
        receiver_username: [u8; 16],
        receiver_transaction_account: Pubkey,
        status: bool,
    ) -> Result<()> {
        self.sender_account = sender_account;
        self.sender_username = sender_username;
        self.mint = mint;
        self.amount = amount;
        self.time_stamp = time_stamp;
        self.receiver_account = receiver_account;
        self.receiver_username = receiver_username;
        self.receiver_transaction_account = receiver_transaction_account;
        self.status = status;
        Ok(())
    }

    fn fulfill(
        &mut self,
        fulfiller: Pubkey,
        fulfiller_username: [u8; 16],
        requester: Pubkey,
        requester_username: [u8; 16],
        status: bool
    ) -> Result<()> {
        self.sender_account = fulfiller;
        self.sender_username = fulfiller_username;
        self.receiver_account = requester;
        self.receiver_username = requester_username;
        self.status = status;
        Ok(())
    }
}