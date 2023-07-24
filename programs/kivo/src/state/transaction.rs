use anchor_lang::prelude::*;

#[account]
pub struct Transaction {
    pub counterparty: Pubkey, 
    pub mint_id: Option<u8>,
    pub amount: u64,
    pub time_stamp: u64,
    pub status: Option<bool>, 
    pub counterparty_tx_seed: u32,
}

impl Transaction {
    pub fn new(
        &mut self,
        counterparty: Pubkey,
        mint_id: Option<u8>,
        amount: u64,
        time_stamp: u64,
        status: Option<bool>,
        counterparty_tx_seed: u32,
    ) -> Result<()> {
        self.counterparty = counterparty;
        self.mint_id = mint_id;
        self.amount = amount;
        self.time_stamp = time_stamp;
        self.status = status;
        self.counterparty_tx_seed = counterparty_tx_seed;
        Ok(())
    }

    pub fn fulfill(
        &mut self,
        counterparty: Pubkey,
        status: bool
    ) -> Result<()> {
        self.counterparty = counterparty;
        self.status = Some(status);
        Ok(())
    }

    pub fn reject(&mut self) {
        self.status = Some(false);
    }

    pub fn get_mint_id(mint: &Pubkey) -> Option<u8> {
        let mint = mint.to_string();
    
        match mint.as_str() {
            "So11111111111111111111111111111111111111112" => Some(1),
            "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU" => Some(2),
            "BhWwL5K6k98xvy2vndXLVvq6vRsnCq9RSM6sCHNPSGMe" => Some(3),
            "G3Cb13RiPcTtdKSfZEyhHCpXkgqyTr9BdVvdUbtERHUR" => Some(4),
            "J9JkoZFdi31nJAcSniPMemfneJ7AL2iMYZkrEC9yvTDK" => Some(5),
            _ => None,
        }
    }
}