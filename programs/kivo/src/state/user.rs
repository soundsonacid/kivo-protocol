use anchor_lang::prelude::*;
// use static_assertions::const_assert_eq;
// use std::mem::size_of;
use crate::state::traits::{ Size, UserAccount, UsernameAccount, FriendAccount };


#[account]
#[derive(Default)]
pub struct User {
    pub owner: Pubkey,
    pub username: [u8; 16], 
    pub account_type: u8, 
    pub total_deposits: u64, 
    pub total_withdraws: u64, 
    pub transactions: u64,
    pub num_friends: u32,
    pub num_contracts: u32,
}

impl User {
    pub(crate) fn set_username(&mut self, username: [u8; 16]) {
        self.username = username;
    }

    pub(crate) fn increment_transactions(&mut self) {
        self.transactions = self.transactions.saturating_add(1);
    }

    pub(crate) fn increment_friends(&mut self) {
        self.num_friends = self.num_friends.saturating_add(1);
    }

    pub(crate) fn increment_contracts(&mut self) {
        self.num_contracts = self.num_contracts.saturating_add(1);
    }

    // pub fn get_user_signer_seeds<'a>(&'a self, pubkey: &'a Pubkey, bump: &'a [u8]) -> [&'a [u8]; 3] {
    //     [b"user", pubkey.as_ref(), bump]
    // }
    
}

impl Size for User {
    const SIZE: usize = 79;
}

impl UserAccount for Account<'_, User> {
    fn new(
        &mut self,
        owner: Pubkey,
        username: [u8; 16],
        account_type: u8,
    ) -> Result<()> {
        self.owner = owner;
        self.username = username;
        self.account_type = account_type;
        Ok(())
    }
}

#[account]
pub struct Username {
    pub user_account: Pubkey,
    pub username: [u8; 16]
}

impl Size for Username {
    const SIZE: usize = 48;
}

impl UsernameAccount for Account<'_, Username> {
    fn new(
        &mut self,
        user_account: Pubkey,
        username: [u8; 16],
    ) -> Result<()> {
        self.user_account = user_account;
        self.username = username;
        Ok(())
    }
}

#[account]
pub struct Friend {
    pub user_account: Pubkey,
    pub friend_account: Pubkey,
}

impl Size for Friend {
    const SIZE: usize = 64;
}

impl FriendAccount for Account<'_, Friend> {
    fn new(
        &mut self,
        user_account: Pubkey,
        friend_account: Pubkey,
    ) -> Result<()> {
        self.user_account = user_account;
        self.friend_account = friend_account;
        Ok(())
    }
}


// const_assert_eq!(User::SIZE, size_of::<User>() + 8);
// const_assert_eq!(Username::SIZE, size_of::<Username>() + 8);
// const_assert_eq!(Friend::SIZE, size_of::<Friend>() + 8);
