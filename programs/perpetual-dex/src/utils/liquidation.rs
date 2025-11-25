use anchor_lang::prelude::*;
use crate::consts::*;

/// Check if position should be liquidated
pub fn should_liquidate(
    current_price: u64,
    liquidation_price: u64,
    side: crate::consts::PositionSide,
) -> bool {
    match side {
        crate::consts::PositionSide::Long => current_price <= liquidation_price,
        crate::consts::PositionSide::Short => current_price >= liquidation_price,
    }
}

/// Calculate liquidation bonus for liquidator
pub fn calculate_liquidation_bonus(
    position_value: u64,
    liquidation_fee_bps: u16,
) -> u64 {
    position_value
        .checked_mul(liquidation_fee_bps as u128)
        .unwrap()
        .checked_div(10000)
        .unwrap() as u64
}

