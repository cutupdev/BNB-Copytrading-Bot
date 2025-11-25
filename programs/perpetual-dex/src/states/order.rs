use anchor_lang::prelude::*;
use crate::consts::*;

#[account]
#[derive(Debug)]
pub struct Order {
    pub order_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub size: u64, // Order size in base units
    pub filled_size: u64, // Filled size
    pub price: u64, // Limit price (for limit orders)
    pub stop_price: Option<u64>, // Stop price (for stop orders)
    pub leverage: u16, // Leverage in basis points
    pub created_at: i64,
    pub updated_at: i64,
}

impl Order {
    pub const SEEDS: &'static [u8] = ORDER_SEED;
    pub const SIZE: usize = 8 + // discriminator
        32 + // order_id
        32 + // user
        32 + // market
        1 +  // side
        1 +  // order_type
        1 +  // status
        8 +  // size
        8 +  // filled_size
        8 +  // price
        9 +  // stop_price (Option<u64>)
        2 +  // leverage
        8 +  // created_at
        8;   // updated_at

    pub fn init(
        &mut self,
        order_id: Pubkey,
        user: Pubkey,
        market: Pubkey,
        side: OrderSide,
        order_type: OrderType,
        size: u64,
        price: u64,
        stop_price: Option<u64>,
        leverage: u16,
        clock: &Clock,
    ) -> Result<()> {
        require!(
            size > 0,
            crate::errors::PerpetualDexError::InvalidOrderSize
        );
        require!(
            price > 0 || order_type == OrderType::Market,
            crate::errors::PerpetualDexError::InvalidPrice
        );
        require!(
            leverage >= MIN_LEVERAGE_BPS && leverage <= MAX_LEVERAGE_BPS,
            crate::errors::PerpetualDexError::InvalidLeverage
        );

        self.order_id = order_id;
        self.user = user;
        self.market = market;
        self.side = side;
        self.order_type = order_type;
        self.status = OrderStatus::Active;
        self.size = size;
        self.filled_size = 0;
        self.price = price;
        self.stop_price = stop_price;
        self.leverage = leverage;
        self.created_at = clock.unix_timestamp;
        self.updated_at = clock.unix_timestamp;

        Ok(())
    }

    pub fn fill(&mut self, fill_size: u64, clock: &Clock) -> Result<()> {
        require!(
            self.status == OrderStatus::Active || self.status == OrderStatus::PartiallyFilled,
            crate::errors::PerpetualDexError::OrderNotActive
        );
        require!(
            self.filled_size + fill_size <= self.size,
            crate::errors::PerpetualDexError::InvalidOrderSize
        );

        self.filled_size += fill_size;
        self.updated_at = clock.unix_timestamp;

        if self.filled_size >= self.size {
            self.status = OrderStatus::Filled;
        } else {
            self.status = OrderStatus::PartiallyFilled;
        }

        Ok(())
    }

    pub fn cancel(&mut self, clock: &Clock) -> Result<()> {
        require!(
            self.status == OrderStatus::Active || self.status == OrderStatus::PartiallyFilled,
            crate::errors::PerpetualDexError::OrderNotActive
        );
        self.status = OrderStatus::Cancelled;
        self.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn get_remaining_size(&self) -> u64 {
        self.size - self.filled_size
    }
}

