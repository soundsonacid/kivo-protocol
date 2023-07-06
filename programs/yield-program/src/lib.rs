use anchor_lang::prelude::*;

declare_id!("7aQcTJCAtyWLxEfysNdSBoshCFU1DyiFhkkzEkNmpSWL");

#[program]
pub mod kivo_yield_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
