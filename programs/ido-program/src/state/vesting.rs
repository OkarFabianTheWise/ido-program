use anchor_lang::prelude::*;

#[account]
pub struct VestingState {
    pub beneficiary: Pubkey,
    pub total_amount: u64,
    pub released_amount: u64,
    pub cliff_end: i64,
    pub vesting_end: i64,
    pub last_claimed: i64,
}
