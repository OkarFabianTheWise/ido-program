use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

// Claim vested tokens
pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let vesting_state = &mut ctx.accounts.vesting_state;

    // Calculate claimable tokens
    let now = Clock::get()?.unix_timestamp;
    let vested_amount = calculate_vested_amount(
        vesting_state.cliff_end,
        vesting_state.vesting_end,
        now,
        vesting_state.total_amount,
    );
    let claimable = vested_amount.saturating_sub(vesting_state.released_amount);
    require!(claimable > 0, ErrorCode::NothingToClaim);

    // Transfer tokens
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.beneficiary_token_account.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), claimable)?;

    // Update vesting state
    vesting_state.released_amount += claimable;
    vesting_state.last_claimed = now;
    Ok(())
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub vesting_state: Account<'info, VestingState>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"vault_authority"], bump)]
    pub vault_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
