use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,
    pub mint_account: Pubkey, // mint account of stable coin
    pub liquidation_threshold: u64, // to check if a collateral account is healty or it should be liquidated
    pub liquidation_bonus: u64, // bonus given to liquidator
    pub min_health_factor: u64, // below this collateral account gets liquidated
    pub bump: u8,
    pub bump_mint_account: u8,
}