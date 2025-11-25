use anchor_lang::prelude::*;

/// Seed for the global configuration PDA
pub const GLOBAL_CONFIG_SEED: &[u8] = b"perpetual_config";

/// Seed for market PDA
pub const MARKET_SEED: &[u8] = b"perpetual_market";

/// Seed for user account PDA
pub const USER_SEED: &[u8] = b"perpetual_user";

/// Seed for position PDA
pub const POSITION_SEED: &[u8] = b"perpetual_position";

/// Seed for order PDA
pub const ORDER_SEED: &[u8] = b"perpetual_order";

/// Maximum leverage (e.g., 100x = 10000 basis points)
pub const MAX_LEVERAGE_BPS: u16 = 10000; // 100x

/// Minimum leverage (1x)
pub const MIN_LEVERAGE_BPS: u16 = 100; // 1x

/// Initial margin requirement in basis points (e.g., 1000 = 10%)
pub const INITIAL_MARGIN_BPS: u16 = 1000; // 10%

/// Maintenance margin requirement in basis points (e.g., 500 = 5%)
pub const MAINTENANCE_MARGIN_BPS: u16 = 500; // 5%

/// Liquidation threshold in basis points (e.g., 400 = 4%)
pub const LIQUIDATION_THRESHOLD_BPS: u16 = 400; // 4%

/// Maximum number of open orders per user
pub const MAX_OPEN_ORDERS: u8 = 50;

/// Maximum position size (in base units)
pub const MAX_POSITION_SIZE: u64 = 1_000_000_000_000_000; // 1 billion

/// Price precision (8 decimals)
pub const PRICE_PRECISION: u64 = 100_000_000; // 10^8

/// Order side enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderSide {
    /// Buy order (long)
    Buy,
    /// Sell order (short)
    Sell,
}

/// Order type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderType {
    /// Market order (execute immediately at best price)
    Market,
    /// Limit order (execute at specified price or better)
    Limit,
    /// Stop loss order
    StopLoss,
    /// Take profit order
    TakeProfit,
}

/// Order status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum OrderStatus {
    /// Order is active and waiting to be filled
    Active,
    /// Order is partially filled
    PartiallyFilled,
    /// Order is fully filled
    Filled,
    /// Order is cancelled
    Cancelled,
    /// Order is rejected
    Rejected,
}

/// Position side enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum PositionSide {
    /// Long position (buy)
    Long,
    /// Short position (sell)
    Short,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Buy
    }
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Active
    }
}

impl Default for PositionSide {
    fn default() -> Self {
        PositionSide::Long
    }
}

