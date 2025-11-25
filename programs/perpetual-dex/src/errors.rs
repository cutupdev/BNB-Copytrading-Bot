use anchor_lang::prelude::*;

#[error_code]
pub enum PerpetualDexError {
    #[msg("Market not found")]
    MarketNotFound,

    #[msg("Position not found")]
    PositionNotFound,

    #[msg("Order not found")]
    OrderNotFound,

    #[msg("Insufficient margin")]
    InsufficientMargin,

    #[msg("Position is not liquidatable")]
    PositionNotLiquidatable,

    #[msg("Unauthorized: Only position owner can perform this action")]
    Unauthorized,

    #[msg("Invalid leverage: Must be between 1x and maximum allowed")]
    InvalidLeverage,

    #[msg("Invalid order size")]
    InvalidOrderSize,

    #[msg("Invalid price")]
    InvalidPrice,

    #[msg("Order already filled or cancelled")]
    OrderNotActive,

    #[msg("Market is paused")]
    MarketPaused,

    #[msg("Position is already closed")]
    PositionClosed,

    #[msg("Slippage exceeded")]
    SlippageExceeded,

    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("Invalid position side")]
    InvalidPositionSide,

    #[msg("Maintenance margin requirement not met")]
    MaintenanceMarginNotMet,

    #[msg("Cannot remove margin: Would violate maintenance margin")]
    CannotRemoveMargin,

    #[msg("Configuration already initialized")]
    AlreadyInitialized,

    #[msg("Invalid configuration parameters")]
    InvalidConfiguration,

    #[msg("Price oracle not updated")]
    PriceOracleStale,

    #[msg("Maximum position size exceeded")]
    MaxPositionSizeExceeded,

    #[msg("Maximum open orders exceeded")]
    MaxOpenOrdersExceeded,
}

