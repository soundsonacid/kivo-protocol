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
    pub fn set_status(&mut self, status: bool) {
        self.status = status;
    }
}

pub trait TransactionAccount {
    fn new(
        &mut self,
        sender_account: Pubkey,
        mint: Pubkey,
        amount: u64,
        time_stamp: u64,
        receiver_transaction_account: Pubkey,
        status: bool
    ) -> Result<()>;
}

impl TransactionAccount for Account<'_, Transaction> {
    fn new(
        &mut self,
        sender_account: Pubkey,
        mint: Pubkey,
        amount: u64,
        time_stamp: u64,
        receiver_transaction_account: Pubkey,
        status: bool,
    ) -> Result<()> {
        self.sender_account = sender_account;
        self.mint = mint;
        self.amount = amount;
        self.time_stamp = time_stamp;
        self.receiver_transaction_account = receiver_transaction_account;
        self.status = status;
        Ok(())
    }
}