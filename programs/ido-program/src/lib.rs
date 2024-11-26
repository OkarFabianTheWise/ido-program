use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
#[allow(unused_imports)]
use state::SaleState;
#[allow(unused_imports)]
use utils::*;

declare_id!("H3dgCqyyyKK5rQUHaFqtSFJ3PctKh9Fp1uhWE8AMc34Y");

#[program]
pub mod ido_program {
    use super::*;

    // Initialize the IDO sale
    pub fn initialize_sale(ctx: Context<InitializeSale>, params: SaleParams) -> Result<()> {
        instructions::initialize_sale(ctx, params)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
        instructions::add_to_whitelist(ctx, user)
    }

    // Claim vested tokens
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim(ctx)
    }

    // Purchase tokens during the sale
    pub fn purchase_tokens(ctx: Context<Purchase>, amount: u64) -> Result<()> {
        instructions::purchase_tokens(ctx, amount)
    }

    // Remove a user from the whitelist
    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
        instructions::remove_from_whitelist(ctx, user)
    }
}
