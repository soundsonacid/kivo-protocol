use anchor_lang::prelude::*;

pub const LENDING_ACCOUNT: &[u8] = b"lending_account";

#[account]
#[derive(Default)]
pub struct LendingAccount {
    pub kivo_account: Pubkey,
    pub marginfi_account: Pubkey,
}

impl LendingAccount {
    pub fn new(
        &mut self,
        kivo_account: Pubkey,
        marginfi_account: Pubkey
    ) -> Result<()> {
        self.kivo_account = kivo_account;
        self.marginfi_account = marginfi_account;
        Ok(())
    }

    pub fn get_lender_signer_seeds<'a>(
        pubkey: &'a Pubkey, 
        bump: &'a u8
    ) -> [&'a [u8]; 3] {
        [LENDING_ACCOUNT.as_ref(), pubkey.as_ref(), bytemuck::bytes_of(bump)]
    }
}