use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, Token2022}};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{burn_tokens, calculate_health_factor, error::ErrorCode, get_lamports_from_usd, withdraw, withdraw_sol, Collateral, Config, SEED_CONFIG_ACCOUNT };

#[derive(Accounts)]
pub struct Liquidate <'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        has_one = sol_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> Liquidate <'info> {
    pub fn liquidate (&mut self, amount_to_burn: u64) -> Result<()> {

        let health_factor = calculate_health_factor(&self.collateral_account, &self.config_account, &self.price_update)?;

        require!(health_factor <= self.config_account.min_health_factor, ErrorCode::AboveMinHealthFactor);

        let lamports = get_lamports_from_usd(&amount_to_burn, &self.price_update)?;
        let liquidation_bonus = lamports * self.config_account.liquidation_bonus / 100;
        let amont_to_liquidate = lamports + liquidation_bonus;
        
        withdraw_sol(
            &self.sol_account,
            &self.liquidator.to_account_info(),
            &self.system_program,
            &self.liquidator.key(),
            amont_to_liquidate,
            self.collateral_account.bump,
        )?;

        burn_tokens(
            &self.mint_account,
            &self.token_program,
            &self.liquidator,
            amount_to_burn,
        )?;

        Ok(())
    }
}