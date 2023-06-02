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

    fn set_username(&mut self, username: [u8; 16]) {
        self.username = username;
    }

    fn increment_transactions(&mut self) {
        self.transactions = self.transactions.saturating_add(1);
    }

    fn increment_friends(&mut self) {
        self.num_friends = self.num_friends.saturating_add(1);
    }

    fn increment_contracts(&mut self) {
        self.num_contracts = self.num_contracts.saturating_add(1);
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
    pub friend_account: Pubkey,
    pub friend_username: [u8; 16],
    pub friend_account_type: u8
}

impl Size for Friend {
    const SIZE: usize = 49;
}

impl FriendAccount for Account<'_, Friend> {
    fn new(
        &mut self,
        friend_account: Pubkey,
        friend_username: [u8; 16],
        friend_account_type: u8
    ) -> Result<()> {
        self.friend_account = friend_account;
        self.friend_username = friend_username;
        self.friend_account_type = friend_account_type;
        Ok(())
    }
}


// const_assert_eq!(User::SIZE, size_of::<User>() + 8);
// const_assert_eq!(Username::SIZE, size_of::<Username>() + 8);
// const_assert_eq!(Friend::SIZE, size_of::<Friend>() + 8);
