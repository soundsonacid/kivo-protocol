use anchor_lang::{ prelude::*, AnchorDeserialize };

pub const CONTRACT: &[u8] = b"contract";
pub const OBLIGOR: &[u8] = b"obligor";
pub const PROPOSAL: &[u8] = b"proposal";

#[account]
#[derive(Debug, Default)]
pub struct Contract {
    pub obligor_user_account: Pubkey,
    pub proposer_user_account: Pubkey,
    pub thread: Option<Pubkey>,
    pub proposal: Pubkey,
    pub mint_id: Option<u8>,
    pub amount: u64,
    pub active: bool,
    pub bump: u8,
    pub num_payments_made: u32,
    pub num_payments_obligated: u32,
    pub id: u32,
}

impl Contract {
    pub fn new(
        &mut self,
        obligor_user_account: Pubkey,
        proposer_user_account: Pubkey,
        proposal: Pubkey,
        mint_id: Option<u8>,
        amount: u64,
        bump: u8,
        num_payments_obligated: u32,
        id: u32,
    ) -> Result<()> {
        self.obligor_user_account = obligor_user_account;
        self.proposer_user_account = proposer_user_account;
        self.proposal = proposal;
        self.mint_id = mint_id;
        self.thread = None;
        self.amount = amount;
        self.active = false;
        self.bump = bump;
        self.num_payments_made = 0;
        self.num_payments_obligated = num_payments_obligated;
        self.id = id;
        Ok(())
    }

    pub fn accept(&mut self, thread: Pubkey) {
        self.active = true;
        self.thread = Some(thread);
    }

    pub fn increment_payments_made(&mut self) {
        self.num_payments_made = self.num_payments_made.saturating_add(1);
    }

    pub fn is_fulfilled(&mut self) -> bool {
        self.num_payments_made == self.num_payments_obligated
    }

    pub fn get_contract_address(receiver: Pubkey, nonce: u32) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                CONTRACT,
                receiver.as_ref(),
                nonce.to_le_bytes().as_ref(),
            ],
            &crate::ID,
        )
    }
}

#[account]
#[derive(Debug, Default)]
pub struct Proposal {
    pub payer_account: Pubkey,
    pub payments_made: u32,
    pub payments_obligated: u32,
    pub status: Option<bool>,
    pub amount: u64,
    pub contract: Pubkey,
    pub mint_id: Option<u8>,
    pub nonce: u32,
}

impl Proposal {
    pub fn new(
        &mut self,
        payer_account: Pubkey,
        payments_obligated: u32,
        amount: u64,
        contract: Pubkey,
        mint_id: Option<u8>,
        nonce: u32,
    ) -> Result<()> {
        self.payer_account = payer_account;
        self.payments_obligated = payments_obligated;
        self.status = None;
        self.amount = amount;
        self.contract = contract;
        self.mint_id = mint_id;
        self.nonce = nonce;

        Ok(())
    }

    pub fn reject(&mut self) {
        self.status = Some(false);
    }

    pub fn accept(&mut self) {
        self.status = Some(true);
    }

    pub fn get_proposal_address(proposer: Pubkey, proposer_num_proposals: u32) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                PROPOSAL,
                proposer.as_ref(),
                proposer_num_proposals.to_le_bytes().as_ref(),
            ],
            &crate::ID,
        )
    }
}

#[account]
#[derive(Debug)]
pub struct Obligor {
    pub user_account: Pubkey,
    pub contract: Pubkey,
    pub active: bool,
    pub last_payment_at: Option<i64>,
    pub bump: u8,
}

impl Obligor {
    pub fn new(
        &mut self,
        user_account: Pubkey,
        contract: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.user_account = user_account;
        self.contract = contract;
        self.active = true;
        self.last_payment_at = None;
        self.bump = bump;

        Ok(())
    }

    pub fn get_obligor_address(obligor: Pubkey, contract: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                OBLIGOR,
                obligor.as_ref(),
                contract.as_ref()
            ],
            &crate::ID,
        )
    }

    pub fn get_obligor_signer_seeds<'a>(
        obligor: &'a Pubkey, 
        contract: &'a Pubkey,
        bump: &'a u8
    ) -> [&'a [u8]; 4] {
        [OBLIGOR.as_ref(), obligor.as_ref(), contract.as_ref(), bytemuck::bytes_of(bump)]
    }
}