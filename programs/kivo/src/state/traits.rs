use anchor_lang::prelude::*;

pub trait Size {
    const SIZE: usize;
}

pub trait TransactionAccount {
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
        status: bool
    ) -> Result<()>;

    fn fulfill(
        &mut self,
        fulfiller: Pubkey,
        fulfiller_username: [u8; 16],
        requester: Pubkey,
        requester_username: [u8; 16],
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

    fn set_username(&mut self, username: [u8; 16]);

    fn increment_transactions(&mut self);

    fn increment_friends(&mut self);

    fn increment_contracts(&mut self);

    fn get_user_signer_seeds<'a>(&'a self, pubkey: &'a Pubkey, bump: &'a u8) -> [&'a [u8]; 3];
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
        friend_account: Pubkey,
        friend_username: [u8; 16],
        friend_account_type: u8
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