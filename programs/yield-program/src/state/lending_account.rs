use anchor_lang::prelude::*;

pub const LENDING_ACCOUNT: &[u8] = b"passive_lending_account";
pub const KIVO_MFI_ACCOUNT: &[u8] = b"kivo_mfi_account";

#[account]
#[derive(Default)]
pub struct PassiveLendingAccount {
    pub kivo_account: Pubkey,
    pub marginfi_account: Pubkey,
    pub marginfi_group: Pubkey,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
}

impl PassiveLendingAccount {
    pub fn new(
        &mut self,
        kivo_account: Pubkey,
        marginfi_account: Pubkey,
        marginfi_group: Pubkey,
    ) -> Result<()> {
        self.kivo_account = kivo_account;
        self.marginfi_account = marginfi_account;
        self.marginfi_group = marginfi_group;
        Ok(())
    }

    pub fn increment_deposits(&mut self, deposit: u64) {
        self.total_deposits = self.total_deposits.saturating_add(deposit);
    }

    pub fn increment_withdrawals(&mut self, withdrawal: u64) {
        self.total_withdrawals = self.total_withdrawals.saturating_add(withdrawal);
    }

    pub fn get_lender_signer_seeds<'a>(
        pubkey: &'a Pubkey, 
        bump: &'a u8
    ) -> [&'a [u8]; 3] {
        [LENDING_ACCOUNT.as_ref(), pubkey.as_ref(), bytemuck::bytes_of(bump)]
    }

    pub fn get_lender_address(pubkey: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                LENDING_ACCOUNT,
                pubkey.as_ref(),
            ],
            &crate::ID,
        )
    }

    pub fn get_mfi_address(pubkey: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                KIVO_MFI_ACCOUNT,
                pubkey.as_ref(),
            ],
            &crate::ID,
        )
    }
}