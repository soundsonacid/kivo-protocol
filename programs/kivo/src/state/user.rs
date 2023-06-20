use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use crate::state::traits::Size;

pub const USER: &[u8] = b"user";

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
    pub preferred_token: Option<Pubkey>,
}

impl Size for User {
    const SIZE: usize = 114;
}

impl User {
    pub fn new(
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

    pub fn set_username(&mut self, username: [u8; 16]) {
        self.username = username;
    }

    pub fn increment_transactions(&mut self) {
        self.transactions = self.transactions.saturating_add(1);
    }

    pub fn increment_friends(&mut self) {
        self.num_friends = self.num_friends.saturating_add(1);
    }

    pub fn increment_contracts(&mut self) {
        self.num_contracts = self.num_contracts.saturating_add(1);
    }

    pub fn increment_withdrawals(&mut self) {
        self.total_withdraws = self.total_withdraws.saturating_add(1);
    }

    pub fn set_preferred_token(&mut self, token: Pubkey) {
        self.preferred_token = Some(token);
    }

    pub fn get_user_signer_seeds<'a>(
        pubkey: &'a Pubkey, 
        bump: &'a u8
    ) -> [&'a [u8]; 3] {
        [USER.as_ref(), pubkey.as_ref(), bytemuck::bytes_of(bump)]
    }

    pub fn get_user_address(pubkey: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                USER,
                pubkey.as_ref(),
            ],
            &crate::ID,
        )
    }
}

#[account]
pub struct Username {
    pub user_account: Pubkey,
    pub username: [u8; 16]
}

impl Size for Username {
    const SIZE: usize = 56;
}

impl Username {
    pub fn new(
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
    const SIZE: usize = 57;
}

impl Friend {
    pub fn new(
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


// const_assert_eq!(User::SIZE, std::mem::size_of::<User>() + 8);
const_assert_eq!(Username::SIZE, std::mem::size_of::<Username>() + 8);
const_assert_eq!(Friend::SIZE, std::mem::size_of::<Friend>() + 8);
