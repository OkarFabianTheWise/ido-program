use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::{prelude::*, solana_program::program_pack::Pack};
use anchor_spl::token_2022::spl_token_2022::state::Mint;

pub fn initialize_sale(ctx: Context<InitializeSale>, params: SaleParams) -> Result<()> {
    let mint_account_info = ctx.accounts.token_mint.to_account_info();
    let mint_data = mint_account_info.data.borrow();

    // Validate token mint account
    let mint = Mint::unpack(&mint_data)?;
    require!(mint.is_initialized, ErrorCode::InvalidTokenMint);

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
    /// CHECK: This is a token mint account, validated in the handler.
    pub token_mint: AccountInfo<'info>,
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
