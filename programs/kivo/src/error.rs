use anchor_lang::prelude::*;

#[error_code]
pub enum KivoError {
    #[msg("Insufficient funds to accept contract!")]
    InsufficientBalanceToAcceptContract,
    #[msg("Failed to reject contract: Bad signer at handle_reject_contract - signer key must match contract.sender!")]
    BadSignerToRejectContract,
    #[msg("Failed to accept contract: Bad signer at handle_accept_contract - signer key must match contract.sender!")]
    BadSignerToAcceptContract,
    #[msg("Username contains invalid characters - Usernames must be 16 characters or less and all lowercase letters or numbers!")]
    InvalidUsername,
    #[msg("Failed to reject request: Bad signer at handle_reject_request - signer key must match requester_transaction_account.fulfiller!")]
    BadSignerToRejectRequest,
}