use anchor_lang::prelude::*;

pub trait Size {
    const SIZE: usize;
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

pub trait UserAccount {
    fn new(
        &mut self,
        owner: Pubkey,
        username: [u8; 16],
        account_type: u8,
    ) -> Result<()>;
}

pub trait UsernameAccount {
    fn new(
        &mut self,
        user_account: Pubkey,
        username: [u8; 16],
    ) -> Result<()>;
}

pub trait FriendAccount {
    fn new(
        &mut self,
        user_account: Pubkey,
        friend_account: Pubkey,
    ) -> Result<()>;
}

pub trait PaymentAccount {
    fn new(
        &mut self,
        amount: u64,
        authority: Pubkey,
        mint: Pubkey,
        receipient: Pubkey,
    ) -> Result<()>;
}