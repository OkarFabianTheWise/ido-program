use anchor_lang::prelude::*;

// Error Codes
#[error_code]
pub enum ErrorCode {
    #[msg("The sale is not active.")]
    SaleNotActive,
    #[msg("The buyer is not whitelisted.")]
    NotWhitelisted,
    #[msg("Insufficient funds to complete the purchase.")]
    InsufficientFunds,
    #[msg("Insufficient tokens available for purchase.")]
    InsufficientTokens,
    #[msg("No claimable tokens.")]
    NothingToClaim,
}
