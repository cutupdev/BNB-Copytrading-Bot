use anchor_lang::prelude::*;
use crate::consts::*;

#[account]
#[derive(Debug)]
pub struct User {
    pub user: Pubkey,
    pub collateral: u64, // Total collateral deposited
    pub used_margin: u64, // Margin currently used in positions
    pub free_margin: u64, // Available margin for new positions
    pub total_pnl: i64, // Total profit and loss
    pub total_positions: u64, // Number of open positions
    pub total_orders: u64, // Number of active orders
    pub created_at: i64,
    pub last_updated: i64,
}

impl User {
    pub const SEEDS: &'static [u8] = USER_SEED;
    pub const SIZE: usize = 8 + // discriminator
        32 + // user
        8 +  // collateral
        8 +  // used_margin
        8 +  // free_margin
        8 +  // total_pnl
        8 +  // total_positions
        8 +  // total_orders
        8 +  // created_at
        8;   // last_updated

    pub fn init(&mut self, user: Pubkey, clock: &Clock) -> Result<()> {
        self.user = user;
        self.collateral = 0;
        self.used_margin = 0;
        self.free_margin = 0;
        self.total_pnl = 0;
        self.total_positions = 0;
        self.total_orders = 0;
        self.created_at = clock.unix_timestamp;
        self.last_updated = clock.unix_timestamp;
        Ok(())
    }

    pub fn deposit_collateral(&mut self, amount: u64, clock: &Clock) -> Result<()> {
        self.collateral = self.collateral.checked_add(amount).unwrap();
        self.free_margin = self.free_margin.checked_add(amount).unwrap();
        self.last_updated = clock.unix_timestamp;
        Ok(())
    }

    pub fn withdraw_collateral(&mut self, amount: u64, clock: &Clock) -> Result<()> {
        require!(
            self.free_margin >= amount,
            crate::errors::PerpetualDexError::InsufficientMargin
        );
        self.collateral = self.collateral.checked_sub(amount).unwrap();
        self.free_margin = self.free_margin.checked_sub(amount).unwrap();
        self.last_updated = clock.unix_timestamp;
        Ok(())
    }

    pub fn use_margin(&mut self, amount: u64, clock: &Clock) -> Result<()> {
        require!(
            self.free_margin >= amount,
            crate::errors::PerpetualDexError::InsufficientMargin
        );
        self.used_margin = self.used_margin.checked_add(amount).unwrap();
        self.free_margin = self.free_margin.checked_sub(amount).unwrap();
        self.last_updated = clock.unix_timestamp;
        Ok(())
    }

    pub fn free_margin(&mut self, amount: u64, clock: &Clock) -> Result<()> {
        self.used_margin = self.used_margin.checked_sub(amount).unwrap();
        self.free_margin = self.free_margin.checked_add(amount).unwrap();
        self.last_updated = clock.unix_timestamp;
        Ok(())
    }

    pub fn increment_positions(&mut self) {
        self.total_positions = self.total_positions.checked_add(1).unwrap();
    }

    pub fn decrement_positions(&mut self) {
        self.total_positions = self.total_positions.checked_sub(1).unwrap();
    }

    pub fn increment_orders(&mut self) {
        self.total_orders = self.total_orders.checked_add(1).unwrap();
    }

    pub fn decrement_orders(&mut self) {
        self.total_orders = self.total_orders.checked_sub(1).unwrap();
    }
}

