use anchor_lang::prelude::*;

#[account]
pub struct Whitelist {
    pub users: Vec<(Pubkey, bool)>,
}
