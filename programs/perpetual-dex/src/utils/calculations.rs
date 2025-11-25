use anchor_lang::prelude::*;
use crate::consts::*;

/// Calculate required margin for a position
pub fn calculate_required_margin(
    size: u64,
    price: u64,
    leverage: u16,
) -> Result<u64> {
    let position_value = (size as u128)
        .checked_mul(price as u128)
        .unwrap()
        .checked_div(PRICE_PRECISION as u128)
        .unwrap();

    let leverage_ratio = (leverage as u128) * PRICE_PRECISION as u128 / 10000;
    let required_margin = position_value
        .checked_mul(PRICE_PRECISION as u128)
        .unwrap()
        .checked_div(leverage_ratio)
        .unwrap();

    Ok(required_margin as u64)
}

/// Calculate profit and loss for a position
pub fn calculate_pnl(
    size: u64,
    entry_price: u64,
    current_price: u64,
    side: crate::consts::PositionSide,
) -> i64 {
    let price_diff = if current_price > entry_price {
        current_price - entry_price
    } else {
        entry_price - current_price
    };

    let pnl_amount = (size as u128)
        .checked_mul(price_diff as u128)
        .unwrap()
        .checked_div(PRICE_PRECISION as u128)
        .unwrap() as u64;

    match side {
        crate::consts::PositionSide::Long => {
            if current_price >= entry_price {
                pnl_amount as i64
            } else {
                -(pnl_amount as i64)
            }
        }
        crate::consts::PositionSide::Short => {
            if current_price <= entry_price {
                pnl_amount as i64
            } else {
                -(pnl_amount as i64)
            }
        }
    }
}

/// Calculate liquidation price
pub fn calculate_liquidation_price(
    entry_price: u64,
    leverage: u16,
    side: crate::consts::PositionSide,
    maintenance_margin_bps: u16,
) -> u64 {
    let leverage_ratio = (leverage as u64) * PRICE_PRECISION / 10000;
    let maintenance_margin_ratio = (maintenance_margin_bps as u64) * PRICE_PRECISION / 10000;

    match side {
        crate::consts::PositionSide::Long => {
            let ratio = PRICE_PRECISION
                .checked_sub(PRICE_PRECISION / leverage_ratio)
                .unwrap()
                .checked_sub(maintenance_margin_ratio)
                .unwrap();
            entry_price
                .checked_mul(ratio)
                .unwrap()
                .checked_div(PRICE_PRECISION)
                .unwrap()
        }
        crate::consts::PositionSide::Short => {
            let ratio = PRICE_PRECISION
                .checked_add(PRICE_PRECISION / leverage_ratio)
                .unwrap()
                .checked_add(maintenance_margin_ratio)
                .unwrap();
            entry_price
                .checked_mul(ratio)
                .unwrap()
                .checked_div(PRICE_PRECISION)
                .unwrap()
        }
    }
}

/// Calculate trading fee
pub fn calculate_trading_fee(amount: u64, fee_bps: u16) -> u64 {
    (amount as u128)
        .checked_mul(fee_bps as u128)
        .unwrap()
        .checked_div(10000)
        .unwrap() as u64
}

