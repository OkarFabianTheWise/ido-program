use crate::errors::ErrorCode;
use crate::state::Whitelist;
use anchor_lang::prelude::*;

// Remove a user from the whitelist
pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    // Check if the user already exists in the whitelist
    // Check if the user exists in the whitelist and remove them
    if let Some(pos) = whitelist.users.iter().position(|(key, _)| *key == user) {
        whitelist.users.remove(pos);
    } else {
        // User not found, handle accordingly (optional)
        // ErrorCode::NotWhitelisted
        return Err(ErrorCode::NotWhitelisted.into());
    }
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
}
