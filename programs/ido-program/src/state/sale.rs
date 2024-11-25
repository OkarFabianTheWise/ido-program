use anchor_lang::prelude::*;

// Account sale States
#[account]
pub struct SaleState {
    pub token_mint: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub token_price: u64,
    pub total_tokens: u64,
}
