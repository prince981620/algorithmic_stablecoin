use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{error::ErrorCode, Collateral, Config, FEED_ID, MAXIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT};

pub fn check_health_factor (
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;

    require!(health_factor >= config.min_health_factor, ErrorCode::BelowMinHealthFactor);

    Ok(())
}

pub fn calculate_health_factor (
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>
) -> Result<u64> {
    let collateral_value_in_usd = get_usd_value(&collateral.lamports_balance, price_feed)?;

    let collateral_adjusted_for_liquidation_threshold = (collateral_value_in_usd * config.liquidation_threshold) / 100;

    if collateral.amount_minted == 0 {
        msg!("Health Factor Max");
        return Ok(u64::MAX);
    }

    let health_factor = (collateral_adjusted_for_liquidation_threshold) / collateral.amount_minted;

    Ok(health_factor)

}


pub fn get_usd_value(
    amount_in_lamports: &u64,
    price_feed: &Account<PriceUpdateV2>
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;
    require!(price.price > 0, ErrorCode::InvalidPrice);

    let price_in_usd =  price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);

    Ok(amount_in_usd as u64)

}