use anchor_lang::prelude::*;
use crate::consts::*;

/// Check if user has sufficient margin
pub fn has_sufficient_margin(
    collateral: u64,
    used_margin: u64,
    required_margin: u64,
) -> bool {
    let free_margin = collateral.checked_sub(used_margin).unwrap_or(0);
    free_margin >= required_margin
}

/// Calculate margin ratio
pub fn calculate_margin_ratio(
    collateral: u64,
    used_margin: u64,
    pnl: i64,
) -> u64 {
    let total_equity = if pnl >= 0 {
        collateral.checked_add(pnl as u64).unwrap_or(collateral)
    } else {
        collateral.checked_sub((-pnl) as u64).unwrap_or(0)
    };

    if used_margin == 0 {
        return PRICE_PRECISION; // 100%
    }

    total_equity
        .checked_mul(PRICE_PRECISION)
        .unwrap()
        .checked_div(used_margin)
        .unwrap()
}

/// Check if position meets maintenance margin requirement
pub fn meets_maintenance_margin(
    margin: u64,
    pnl: i64,
    maintenance_margin_bps: u16,
) -> bool {
    let equity = if pnl >= 0 {
        margin.checked_add(pnl as u64).unwrap_or(margin)
    } else {
        margin.checked_sub((-pnl) as u64).unwrap_or(0)
    };

    let required_maintenance = margin
        .checked_mul(maintenance_margin_bps as u64)
        .unwrap()
        .checked_div(10000)
        .unwrap();

    equity >= required_maintenance
}

