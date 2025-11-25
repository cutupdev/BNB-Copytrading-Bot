use anchor_lang::prelude::*;
use crate::consts::*;

#[account]
#[derive(Debug)]
pub struct Position {
    pub position_id: Pubkey,
    pub user: Pubkey,
    pub market: Pubkey,
    pub side: PositionSide,
    pub size: u64, // Position size in base units
    pub entry_price: u64, // Entry price
    pub current_price: u64, // Current market price
    pub leverage: u16, // Leverage in basis points (e.g., 1000 = 10x)
    pub margin: u64, // Collateral margin
    pub liquidation_price: u64, // Price at which position will be liquidated
    pub pnl: i64, // Current profit and loss
    pub funding_payments: i64, // Cumulative funding payments
    pub opened_at: i64,
    pub last_updated: i64,
    pub is_closed: bool,
}

impl Position {
    pub const SEEDS: &'static [u8] = POSITION_SEED;
    pub const SIZE: usize = 8 + // discriminator
        32 + // position_id
        32 + // user
        32 + // market
        1 +  // side
        8 +  // size
        8 +  // entry_price
        8 +  // current_price
        2 +  // leverage
        8 +  // margin
        8 +  // liquidation_price
        8 +  // pnl
        8 +  // funding_payments
        8 +  // opened_at
        8 +  // last_updated
        1;   // is_closed

    pub fn init(
        &mut self,
        position_id: Pubkey,
        user: Pubkey,
        market: Pubkey,
        side: PositionSide,
        size: u64,
        entry_price: u64,
        leverage: u16,
        margin: u64,
        clock: &Clock,
    ) -> Result<()> {
        require!(
            size > 0,
            crate::errors::PerpetualDexError::InvalidOrderSize
        );
        require!(
            entry_price > 0,
            crate::errors::PerpetualDexError::InvalidPrice
        );
        require!(
            leverage >= MIN_LEVERAGE_BPS && leverage <= MAX_LEVERAGE_BPS,
            crate::errors::PerpetualDexError::InvalidLeverage
        );
        require!(
            margin > 0,
            crate::errors::PerpetualDexError::InsufficientMargin
        );

        self.position_id = position_id;
        self.user = user;
        self.market = market;
        self.side = side;
        self.size = size;
        self.entry_price = entry_price;
        self.current_price = entry_price;
        self.leverage = leverage;
        self.margin = margin;
        self.liquidation_price = self.calculate_liquidation_price(entry_price, leverage, side);
        self.pnl = 0;
        self.funding_payments = 0;
        self.opened_at = clock.unix_timestamp;
        self.last_updated = clock.unix_timestamp;
        self.is_closed = false;

        Ok(())
    }

    pub fn calculate_liquidation_price(&self, entry_price: u64, leverage: u16, side: PositionSide) -> u64 {
        // Simplified liquidation price calculation
        // For long: liquidation_price = entry_price * (1 - 1/leverage - maintenance_margin)
        // For short: liquidation_price = entry_price * (1 + 1/leverage + maintenance_margin)
        let leverage_ratio = (leverage as u64) * PRICE_PRECISION / 10000;
        let maintenance_margin_ratio = (MAINTENANCE_MARGIN_BPS as u64) * PRICE_PRECISION / 10000;
        
        match side {
            PositionSide::Long => {
                let ratio = PRICE_PRECISION - (PRICE_PRECISION / leverage_ratio) - maintenance_margin_ratio;
                entry_price * ratio / PRICE_PRECISION
            }
            PositionSide::Short => {
                let ratio = PRICE_PRECISION + (PRICE_PRECISION / leverage_ratio) + maintenance_margin_ratio;
                entry_price * ratio / PRICE_PRECISION
            }
        }
    }

    pub fn update_pnl(&mut self, current_price: u64) -> Result<()> {
        self.current_price = current_price;
        
        let price_diff = if current_price > self.entry_price {
            current_price - self.entry_price
        } else {
            self.entry_price - current_price
        };

        let pnl_amount = (self.size as u128)
            .checked_mul(price_diff as u128)
            .unwrap()
            .checked_div(PRICE_PRECISION as u128)
            .unwrap() as u64;

        self.pnl = match self.side {
            PositionSide::Long => {
                if current_price >= self.entry_price {
                    pnl_amount as i64
                } else {
                    -(pnl_amount as i64)
                }
            }
            PositionSide::Short => {
                if current_price <= self.entry_price {
                    pnl_amount as i64
                } else {
                    -(pnl_amount as i64)
                }
            }
        };

        self.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn is_liquidatable(&self, current_price: u64) -> bool {
        match self.side {
            PositionSide::Long => current_price <= self.liquidation_price,
            PositionSide::Short => current_price >= self.liquidation_price,
        }
    }

    pub fn close(&mut self, clock: &Clock) -> Result<()> {
        require!(
            !self.is_closed,
            crate::errors::PerpetualDexError::PositionClosed
        );
        self.is_closed = true;
        self.last_updated = clock.unix_timestamp;
        Ok(())
    }
}

