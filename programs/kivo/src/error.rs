use anchor_lang::prelude::*;

#[error_code]
pub enum KivoError {
    #[msg("Transaction signer is not request fulfiller")]
    BadSignerToRejectRequest,
    #[msg("Attempted to withdraw an amount exceeding user balance")]
    GroupWithdrawalExceedsBalance,
    #[msg("Attemped to use an amount exceeding user balance")]
    ModeUsageExceedsBalance,
    #[msg("Output token account balance change is LTE 0")]
    NegDelta,
}