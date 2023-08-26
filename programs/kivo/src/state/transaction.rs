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

    pub fn fulfill(&mut self) {
        self.status = Some(true);
    }

    pub fn reject(&mut self) {
        self.status = Some(false);
    }

    pub fn get_mint_id(mint: &Pubkey) -> Option<u8> {
        let mint = mint.to_string();
    
        match mint.as_str() {
            "So11111111111111111111111111111111111111112" => Some(1),
            "8kKGD6dQ6mhr9YBx3T4oRGoFCJfnpLyRBGdV1upWRYnq" => Some(2),
            "BhWwL5K6k98xvy2vndXLVvq6vRsnCq9RSM6sCHNPSGMe" => Some(3),
            "G3Cb13RiPcTtdKSfZEyhHCpXkgqyTr9BdVvdUbtERHUR" => Some(4),
            "J9JkoZFdi31nJAcSniPMemfneJ7AL2iMYZkrEC9yvTDK" => Some(5),
            _ => None,
        }
    }
}