use anchor_lang::prelude::*;

use crate::{Config, SEED_CONFIG_ACCOUNT};


#[derive(Accounts)]
pub struct UpdateConfig <'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = authority,
    )]
    pub config_account: Account<'info, Config>
}

impl <'info> UpdateConfig <'info> {
    pub fn update_config (&mut self, min_health_factor: u64) -> Result<()> {
        self.config_account.min_health_factor = min_health_factor;
         Ok(())
    }
}