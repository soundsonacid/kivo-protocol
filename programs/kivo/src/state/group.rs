use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Group {
    pub admin: Pubkey,
    pub identifier: u8, 
}

impl Group {
    pub fn new(
        &mut self,
        admin: Pubkey,
        identifier: u8
    ) -> Result<()> {
        self.admin = admin;
        self.identifier = identifier;
        Ok(())
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