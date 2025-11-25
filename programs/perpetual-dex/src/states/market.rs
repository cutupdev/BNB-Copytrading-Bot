use anchor_lang::prelude::*;
use crate::consts::*;

#[account]
#[derive(Debug)]
pub struct Market {
    pub market_id: Pubkey,
    pub base_mint: Pubkey, // Base asset (e.g., SOL)
    pub quote_mint: Pubkey, // Quote asset (e.g., USDC)
    pub name: String,
    pub symbol: String,
    pub current_price: u64, // Current price in quote units (scaled by PRICE_PRECISION)
    pub last_update_time: i64,
    pub total_long_size: u64, // Total long position size
    pub total_short_size: u64, // Total short position size
    pub open_interest_long: u64, // Open interest for long positions
    pub open_interest_short: u64, // Open interest for short positions
    pub funding_rate: i64, // Funding rate in basis points (can be negative)
    pub next_funding_time: i64, // Next funding payment time
    pub paused: bool,
    pub created_at: i64,
}

impl Market {
    pub const SEEDS: &'static [u8] = MARKET_SEED;
    pub const BASE_SIZE: usize = 8 + // discriminator
        32 + // market_id
        32 + // base_mint
        32 + // quote_mint
        4 +  // name (String length prefix)
        4 +  // symbol (String length prefix)
        8 +  // current_price
        8 +  // last_update_time
        8 +  // total_long_size
        8 +  // total_short_size
        8 +  // open_interest_long
        8 +  // open_interest_short
        8 +  // funding_rate
        8 +  // next_funding_time
        1 +  // paused
        8;   // created_at

    pub fn init(
        &mut self,
        market_id: Pubkey,
        base_mint: Pubkey,
        quote_mint: Pubkey,
        name: String,
        symbol: String,
        initial_price: u64,
        clock: &Clock,
    ) -> Result<()> {
        require!(
            name.len() <= 50,
            crate::errors::PerpetualDexError::InvalidConfiguration
        );
        require!(
            symbol.len() <= 10,
            crate::errors::PerpetualDexError::InvalidConfiguration
        );

        self.market_id = market_id;
        self.base_mint = base_mint;
        self.quote_mint = quote_mint;
        self.name = name;
        self.symbol = symbol;
        self.current_price = initial_price;
        self.last_update_time = clock.unix_timestamp;
        self.total_long_size = 0;
        self.total_short_size = 0;
        self.open_interest_long = 0;
        self.open_interest_short = 0;
        self.funding_rate = 0;
        self.next_funding_time = clock.unix_timestamp + 3600; // 1 hour default
        self.paused = false;
        self.created_at = clock.unix_timestamp;

        Ok(())
    }

    pub fn update_price(&mut self, new_price: u64, clock: &Clock) -> Result<()> {
        require!(
            new_price > 0,
            crate::errors::PerpetualDexError::InvalidPrice
        );
        self.current_price = new_price;
        self.last_update_time = clock.unix_timestamp;
        Ok(())
    }

    pub fn update_open_interest(&mut self, long_delta: i64, short_delta: i64) -> Result<()> {
        if long_delta > 0 {
            self.open_interest_long = self.open_interest_long
                .checked_add(long_delta as u64)
                .ok_or(crate::errors::PerpetualDexError::InvalidConfiguration)?;
        } else if long_delta < 0 {
            self.open_interest_long = self.open_interest_long
                .checked_sub((-long_delta) as u64)
                .ok_or(crate::errors::PerpetualDexError::InvalidConfiguration)?;
        }

        if short_delta > 0 {
            self.open_interest_short = self.open_interest_short
                .checked_add(short_delta as u64)
                .ok_or(crate::errors::PerpetualDexError::InvalidConfiguration)?;
        } else if short_delta < 0 {
            self.open_interest_short = self.open_interest_short
                .checked_sub((-short_delta) as u64)
                .ok_or(crate::errors::PerpetualDexError::InvalidConfiguration)?;
        }

        Ok(())
    }
}

