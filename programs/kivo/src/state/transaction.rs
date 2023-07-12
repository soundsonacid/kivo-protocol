use anchor_lang::prelude::*;

#[account]
pub struct Transaction {
    pub sender_account: Pubkey, 
    pub sender_username: [u8; 16],
    pub mint: Pubkey,
    pub amount: u64,
    pub time_stamp: u64,
    pub receiver_account: Pubkey,
    pub receiver_username: [u8; 16],
    pub receiver_transaction_account: Pubkey,
    pub status: bool 
}

impl Transaction {
    pub fn new(
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

    pub fn fulfill(
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