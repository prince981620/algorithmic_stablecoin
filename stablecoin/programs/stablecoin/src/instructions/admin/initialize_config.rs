use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::Mint};

use crate::{
    Config, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS, MIN_HEALTH_FACTOR,
    SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT,
};

#[derive(Accounts)]
pub struct InitialzeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitialzeConfig<'info> {
    pub fn initialize_config(&mut self, bump: InitialzeConfigBumps) -> Result<()> {
        self.config_account.set_inner(Config {
            authority: self.authority.key(),
            mint_account: self.mint_account.key(),
            liquidation_threshold: LIQUIDATION_THRESHOLD,
            liquidation_bonus: LIQUIDATION_BONUS,
            min_health_factor: MIN_HEALTH_FACTOR,
            bump: bump.config_account,
            bump_mint_account: bump.mint_account,
        });

        Ok(())
    }
}
