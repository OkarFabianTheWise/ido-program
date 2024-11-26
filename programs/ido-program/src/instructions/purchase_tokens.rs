use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

// Purchase tokens during the sale
pub fn purchase_tokens(ctx: Context<Purchase>, amount: u64) -> Result<()> {
    let sale_state = &mut ctx.accounts.sale_state;

    // Validate sale period
    let now = Clock::get()?.unix_timestamp;
    require!(
        now >= sale_state.start_time && now <= sale_state.end_time,
        ErrorCode::SaleNotActive
    );

    // Validate whitelist
    let whitelist = &ctx.accounts.whitelist;
    require!(
        whitelist
            .users
            .iter()
            .any(|(key, _)| *key == ctx.accounts.buyer.key()),
        ErrorCode::NotWhitelisted
    );

    // Validate token availability
    require!(
        sale_state.sold_tokens + amount <= sale_state.total_tokens,
        ErrorCode::InsufficientTokens
    );

    // Calculate cost and transfer funds
    let cost = amount * sale_state.token_price;
    let buyer_lamports = ctx.accounts.buyer.lamports();
    require!(buyer_lamports >= cost, ErrorCode::InsufficientFunds);

    **ctx
        .accounts
        .buyer
        .to_account_info()
        .try_borrow_mut_lamports()? -= cost;
    **ctx
        .accounts
        .sale_vault
        .to_account_info()
        .try_borrow_mut_lamports()? += cost;

    // Update sale state
    sale_state.sold_tokens += amount;

    // Lock purchased tokens into vesting
    let vesting_state = &mut ctx.accounts.vesting_state;
    vesting_state.beneficiary = ctx.accounts.buyer.key();
    vesting_state.total_amount += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub sale_state: Account<'info, SaleState>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub sale_vault: AccountInfo<'info>,
    #[account(init, payer = buyer, space = 8 + std::mem::size_of::<VestingState>())]
    pub vesting_state: Account<'info, VestingState>,
    pub system_program: Program<'info, System>,
}
