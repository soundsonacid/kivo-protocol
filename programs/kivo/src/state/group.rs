use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Group {
    pub group_id: u32,
    pub group_name: [u8; 32],
    pub admin: Pubkey,
    pub num_members: u8,
}

#[account]
#[derive(Default)]
pub struct PaidGroup {
    pub group_id: u32,
    pub group_name: [u8; 32],
    pub admin: Pubkey,
    pub num_members: u8,
    pub fee: u64,
    pub recurring: bool,
}

impl Group {
    pub fn new(
        &mut self,
        group_id: u32,
        group_name: [u8; 32],
        admin: Pubkey,
    ) -> Result<()> {
        self.group_id = group_id;
        self.group_name = group_name;
        self.admin = admin;
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
}

impl PaidGroup {
    pub fn new(
        &mut self,
        group_id: u32,
        group_name: [u8; 32],
        admin: Pubkey,
        fee: u64,
        recurring: bool
    ) -> Result<()> {
        self.group_id = group_id;
        self.group_name = group_name;
        self.admin = admin;
        self.fee = fee;
        self.recurring = recurring;
        Ok(())
    }

    pub fn transfer_ownership(
        &mut self,
        new_admin: Pubkey,
    ) -> Result<()> {
        self.admin = new_admin;
        Ok(())
    }

    pub fn change_fee(
        &mut self, 
        fee: u64
    ) -> Result<()> {
        self.fee = fee;
        Ok(())
        }

    pub fn increment_members(&mut self) {
        self.num_members = self.num_members.saturating_add(1);
    }

    pub fn decrement_members(&mut self) {
        self.num_members = self.num_members.saturating_sub(1);
    }
}

#[account]
pub struct Membership {
    pub member: Pubkey,
    pub group: Pubkey,
}

impl Membership {
    pub fn new(
        &mut self,
        member: Pubkey,
        group: Pubkey
    ) -> Result<()> {
        self.member = member;
        self.group = group;
        Ok(())
    }
}

#[account]
pub struct Invite {
    pub invitee: Pubkey,
    pub group: Pubkey,
}

impl Invite {
    pub fn new(
        &mut self,
        invitee: Pubkey,
        group: Pubkey,
    ) -> Result<()> {
        self.invitee = invitee;
        self.group = group;
        Ok(())
    }
}