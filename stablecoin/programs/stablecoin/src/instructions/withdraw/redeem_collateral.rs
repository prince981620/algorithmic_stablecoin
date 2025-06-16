use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    burn_tokens, check_health_factor, withdraw_sol, Collateral, Config, SEED_COLLATERAL_ACCOUNT,
    SEED_CONFIG_ACCOUNT,
};

#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = sol_account,
        has_one = token_account
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> RedeemCollateralAndBurnTokens<'info> {
    pub fn redeem_collateral_and_burn_tokens(
        &mut self,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        // self.collateral_account.lamports_balance = self.sol_account.lamports() - amount_collateral;
        // self.collateral_account.amount_minted -= amount_to_burn;

        check_health_factor(
            &self.collateral_account,
            &self.config_account,
            &self.price_update,
        )?;

        burn_tokens(
            &self.mint_account,
            &self.token_program,
            &self.depositor,
            amount_to_burn,
        )?;

        withdraw_sol(
            &self.sol_account,
            &self.depositor.to_account_info(),
            &self.system_program,
            &self.depositor.key(),
            amount_collateral,
            self.collateral_account.bump_sol_account,
        )?;
        

        Ok(())
    }
}
