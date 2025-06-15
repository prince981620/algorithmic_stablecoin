pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7YYzjV1PiTy7X2yCMQqUuAoFeyr2yFVAHCSEsZmHjdEp");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitialzeConfig>) -> Result<()> {
        ctx.accounts.initialize_config(ctx.bumps)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        ctx.accounts.update_config(min_health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(ctx: Context<DepositCollateralAndMintTokens>, amount_collateral: u64, amount_to_mint: u64) -> Result<()> {
        ctx.accounts.deposit_collateral_and_mint_tokens(amount_collateral, amount_to_mint, ctx.bumps)
    }

    pub fn redeem_collateral_and_burn_tokens(ctx: Context<RedeemCollateralAndBurnTokens>, amount_collateral: u64, amount_to_burn: u64) -> Result<()> {
        ctx.accounts.redeem_collateral_and_burn_tokens(amount_collateral, amount_to_burn)
    }

    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        ctx.accounts.liquidate(amount_to_burn)
    }


}
