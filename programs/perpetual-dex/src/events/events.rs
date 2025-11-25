use anchor_lang::prelude::*;
use crate::consts::{OrderSide, OrderType, PositionSide};

#[event]
pub struct MarketCreated {
    pub market: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub name: String,
    pub created_at: i64,
}

#[event]
pub struct OrderPlaced {
    pub order_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: u64,
    pub price: u64,
    pub leverage: u16,
    pub created_at: i64,
}

#[event]
pub struct OrderCancelled {
    pub order_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub cancelled_at: i64,
}

#[event]
pub struct OrderFilled {
    pub order_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub filled_size: u64,
    pub fill_price: u64,
    pub filled_at: i64,
}

#[event]
pub struct PositionOpened {
    pub position_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub side: PositionSide,
    pub size: u64,
    pub entry_price: u64,
    pub leverage: u16,
    pub margin: u64,
    pub opened_at: i64,
}

#[event]
pub struct PositionClosed {
    pub position_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub exit_price: u64,
    pub pnl: i64,
    pub closed_at: i64,
}

#[event]
pub struct MarginAdded {
    pub position_id: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub updated_at: i64,
}

#[event]
pub struct MarginRemoved {
    pub position_id: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub updated_at: i64,
}

#[event]
pub struct PositionLiquidated {
    pub position_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub liquidation_price: u64,
    pub pnl: i64,
    pub liquidated_at: i64,
}

#[event]
pub struct CollateralDeposited {
    pub user: Pubkey,
    pub amount: u64,
    pub deposited_at: i64,
}

#[event]
pub struct CollateralWithdrawn {
    pub user: Pubkey,
    pub amount: u64,
    pub withdrawn_at: i64,
}

#[event]
pub struct PriceUpdated {
    pub market: Pubkey,
    pub old_price: u64,
    pub new_price: u64,
    pub updated_at: i64,
}

