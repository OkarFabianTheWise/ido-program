use crate::state::Whitelist;
use anchor_lang::prelude::*;

// Remove a user from the whitelist
pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.users.remove(&user);
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveFromWhitelist<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
}
