use anchor_lang::prelude::*;
use crate::consts::*;

#[account]
#[derive(Debug)]
pub struct GlobalConfig {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub trading_fee_bps: u16, // Trading fee in basis points
    pub liquidation_fee_bps: u16, // Liquidation fee in basis points
    pub max_leverage_bps: u16, // Maximum leverage in basis points
    pub initial_margin_bps: u16, // Initial margin requirement
    pub maintenance_margin_bps: u16, // Maintenance margin requirement
    pub liquidation_threshold_bps: u16, // Liquidation threshold
    pub is_initialized: bool,
    pub total_markets: u64,
    pub paused: bool,
}

impl GlobalConfig {
    pub const SEEDS: &'static [u8] = GLOBAL_CONFIG_SEED;
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        32 + // fee_recipient
        2 +  // trading_fee_bps
        2 +  // liquidation_fee_bps
        2 +  // max_leverage_bps
        2 +  // initial_margin_bps
        2 +  // maintenance_margin_bps
        2 +  // liquidation_threshold_bps
        1 +  // is_initialized
        8 +  // total_markets
        1;   // paused

    pub fn init(
        &mut self,
        authority: Pubkey,
        fee_recipient: Pubkey,
        trading_fee_bps: u16,
        liquidation_fee_bps: u16,
        max_leverage_bps: u16,
        initial_margin_bps: u16,
        maintenance_margin_bps: u16,
        liquidation_threshold_bps: u16,
    ) -> Result<()> {
        require!(!self.is_initialized, crate::errors::PerpetualDexError::AlreadyInitialized);
        require!(max_leverage_bps <= MAX_LEVERAGE_BPS, crate::errors::PerpetualDexError::InvalidConfiguration);
        require!(trading_fee_bps <= 10000, crate::errors::PerpetualDexError::InvalidConfiguration);
        require!(liquidation_fee_bps <= 10000, crate::errors::PerpetualDexError::InvalidConfiguration);

        self.authority = authority;
        self.fee_recipient = fee_recipient;
        self.trading_fee_bps = trading_fee_bps;
        self.liquidation_fee_bps = liquidation_fee_bps;
        self.max_leverage_bps = max_leverage_bps;
        self.initial_margin_bps = initial_margin_bps;
        self.maintenance_margin_bps = maintenance_margin_bps;
        self.liquidation_threshold_bps = liquidation_threshold_bps;
        self.is_initialized = true;
        self.total_markets = 0;
        self.paused = false;

        Ok(())
    }

    pub fn increment_total_markets(&mut self) {
        self.total_markets = self.total_markets.checked_add(1).unwrap();
    }
}

