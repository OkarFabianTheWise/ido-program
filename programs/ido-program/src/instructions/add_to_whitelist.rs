use crate::state::Whitelist;
use anchor_lang::prelude::*;

// Add a user to the whitelist
pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.users.insert(user, true);
    Ok(())
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
}
