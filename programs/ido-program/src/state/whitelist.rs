use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[account]
pub struct Whitelist {
    pub users: BTreeMap<Pubkey, bool>,
}
