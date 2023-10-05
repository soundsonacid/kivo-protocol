use anchor_lang::prelude::*;

#[account]
pub struct Transaction {
    pub user_1: Pubkey,
    pub user_2: Pubkey,
    pub amt: u64,
    pub status: Option<bool>,
}

impl Transaction {
    pub fn new(
        &mut self,
        user_1: Pubkey,
        user_2: Pubkey,
        amt: u64,
        status: Option<bool>
    ) -> Result<()> {
        self.user_1 = user_1;
        self.user_2 = user_2;
        self.amt = amt;
        self.status = status;
        Ok(())
    }

    pub fn fulfill(&mut self, amt_final: u64) {
        self.status = Some(true);
        self.amt = amt_final;
    }

    pub fn reject(&mut self) {
        self.status = Some(false);
    }
}