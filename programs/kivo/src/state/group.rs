use anchor_lang::prelude::*;

use crate::constants::GROUP;

#[account]
#[derive(Default)]
pub struct Group {
    pub group_id: u32,
    pub group_name: [u8; 32],
    pub admin: Pubkey,
    pub num_members: u8,
    pub identifier: u8, 
}

impl Group {
    pub fn new(
        &mut self,
        group_id: u32,
        group_name: [u8; 32],
        admin: Pubkey,
        identifier: u8
    ) -> Result<()> {
        self.group_id = group_id;
        self.group_name = group_name;
        self.admin = admin;
        self.identifier = identifier;
        Ok(())
    }

    pub fn transfer_ownership(
        &mut self,
        new_admin: Pubkey,
    ) -> Result<()> {
        self.admin = new_admin;
        Ok(())
    }

    pub fn increment_members(&mut self) {
        self.num_members = self.num_members.saturating_add(1);
    }

    pub fn decrement_members(&mut self) {
        self.num_members = self.num_members.saturating_sub(1);
    }

    pub fn get_group_address(pubkey: Pubkey, identifier: u8) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                GROUP,
                pubkey.as_ref(),
                &identifier.to_le_bytes()
            ], 
            &crate::ID)
    }

    pub fn get_group_signer_seeds<'a>(
        pubkey: &'a Pubkey, 
        identifier: &'a [u8],
        bump: &'a [u8]         
    ) -> [&'a [u8]; 4] {
        [GROUP.as_ref(), pubkey.as_ref(), identifier, bump]
    }
}



#[account]
#[derive(Default)]
pub struct Balance {
    pub member: Pubkey,
    pub group: Pubkey,
    pub mint: Pubkey,
    pub balance: u64,
    pub initialized: bool,
}


impl Balance {
    pub fn new(
        &mut self,
        member: Pubkey,
        group: Pubkey,
        mint: Pubkey,
    ) -> Result<()> {
        self.member = member;
        self.group = group;
        self.mint = mint;
        self.balance = 0;
        self.initialized = true;
        Ok(())
    }

    pub fn increment_balance(&mut self, deposit: u64) {
        self.balance = self.balance.saturating_add(deposit);
    }

    pub fn decrement_balance(&mut self, wd: u64) {
        self.balance = self.balance.saturating_sub(wd);
    }
}