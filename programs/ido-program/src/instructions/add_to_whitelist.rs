use crate::state::Whitelist;
use anchor_lang::prelude::*;

// Add a user to the whitelist
pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;

    // Check if the user already exists in the whitelist
    if !whitelist.users.iter().any(|(key, _)| *key == user) {
        // Add the user to the whitelist
        whitelist.users.push((user, true));
    }

    Ok(())
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
}
