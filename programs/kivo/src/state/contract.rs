use anchor_lang::{ prelude::*, AnchorDeserialize };
use std::convert::TryFrom;

use crate::state::traits::Size;

#[account]
#[derive(Debug)]
pub struct Payment {
    pub amount: u64,
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub receipient: Pubkey,
}

impl Payment {
    pub fn pubkey(authority: Pubkey, mint: Pubkey, receipient: Pubkey, nonce: u32) -> Pubkey {
        Pubkey::find_program_address(
            &[
                b"payment",
                authority.as_ref(),
                mint.as_ref(),
                receipient.as_ref(),
                nonce.to_le_bytes().as_ref(),
            ],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Payment {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Payment::try_deserialize(&mut data.as_slice())
    }
}

impl Size for Payment {
    const SIZE: usize = 104;
}

impl Payment {
    pub fn new(
        &mut self,
        amount: u64,
        authority: Pubkey,
        mint: Pubkey,
        receipient: Pubkey,
    ) -> Result<()> {
        self.amount = amount;
        self.authority = authority;
        self.mint = mint;
        self.receipient = receipient;
        Ok(())
    }
}