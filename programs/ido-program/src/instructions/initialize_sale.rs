use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

// Initialize the IDO sale
pub fn initialize_sale(ctx: Context<InitializeSale>, params: SaleParams) -> Result<()> {
    let sale_state = &mut ctx.accounts.sale_state;
    sale_state.token_mint = ctx.accounts.token_mint.key();
    sale_state.start_time = params.start_time;
    sale_state.end_time = params.end_time;
    sale_state.token_price = params.token_price;
    sale_state.total_tokens = params.total_tokens;
    sale_state.sold_tokens = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSale<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + std::mem::size_of::<SaleState>())]
    pub sale_state: Account<'info, SaleState>,
    #[account(init, payer = admin, space = 8 + 32 + 8)] // Adjust space as needed
    pub whitelist: Account<'info, Whitelist>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

// Sale Parameters
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct SaleParams {
    pub start_time: i64,
    pub end_time: i64,
    pub token_price: u64,
    pub total_tokens: u64,
}
