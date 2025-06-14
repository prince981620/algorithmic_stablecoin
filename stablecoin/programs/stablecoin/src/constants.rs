use anchor_lang::prelude::*;

pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";
pub const SEED_MINT_ACCOUNT: &[u8] = b"mint";
pub const SEED_COLLATERAL_ACCOUNT: &[u8] = b"collateral";
pub const SEED_SOL_ACCOUNT: &[u8] = b"sol";

#[constant]
pub const FEED_ID: &str = "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE";
pub const PRICE_FEED_DECIMAL_ADJUSTMENT: u128 = 10;
pub const MAXIMUM_AGE: u64 = 100;
pub const MINT_DECIMALS: u8 = 9;
pub const LIQUIDATION_THRESHOLD: u64 = 50; // 200% over collateralised
pub const LIQUIDATION_BONUS: u64 = 10; 
pub const MIN_HEALTH_FACTOR: u64 = 1;
