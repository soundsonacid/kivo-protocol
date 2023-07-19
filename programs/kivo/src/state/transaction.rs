use anchor_lang::prelude::*;

#[account]
pub struct Transaction {
    pub sender_account: Pubkey, 
    pub mint_id: Option<u8>,
    pub amount: u64,
    pub time_stamp: u64,
    pub receiver_account: Pubkey,
    pub status: Option<bool>, 
    pub requester_tx_seed: u32,
}

impl Transaction {
    pub fn new(
        &mut self,
        sender_account: Pubkey,
        mint_id: Option<u8>,
        amount: u64,
        time_stamp: u64,
        receiver_account: Pubkey,
        status: Option<bool>,
        requester_tx_seed: u32,
    ) -> Result<()> {
        self.sender_account = sender_account;
        self.mint_id = mint_id;
        self.amount = amount;
        self.time_stamp = time_stamp;
        self.receiver_account = receiver_account;
        self.status = status;
        self.requester_tx_seed = requester_tx_seed;
        Ok(())
    }

    pub fn fulfill(
        &mut self,
        fulfiller: Pubkey,
        requester: Pubkey,
        status: bool
    ) -> Result<()> {
        self.sender_account = fulfiller;
        self.receiver_account = requester;
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