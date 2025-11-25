use anchor_lang::prelude::*;
use crate::states::{User, Position};
use crate::events::MarginRemoved;
use crate::errors::PerpetualDexError;
use crate::utils::meets_maintenance_margin;

#[derive(Accounts)]
pub struct RemoveMargin<'info> {
    #[account(
        mut,
        seeds = [User::SEEDS, user.user.as_ref()],
        bump,
        constraint = user.user == trader.key() @ PerpetualDexError::Unauthorized
    )]
    pub user: Box<Account<'info, User>>,

    #[account(
        mut,
        seeds = [Position::SEEDS, position.position_id.as_ref()],
        bump,
        constraint = position.user == trader.key() @ PerpetualDexError::Unauthorized,
        constraint = !position.is_closed @ PerpetualDexError::PositionClosed
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(mut)]
    pub trader: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> RemoveMargin<'info> {
    pub fn process(&mut self, _position_id: u64, amount: u64) -> Result<()> {
        require!(
            amount > 0,
            PerpetualDexError::InsufficientMargin
        );

        require!(
            self.position.margin > amount,
            PerpetualDexError::InsufficientMargin
        );

        // Update PnL first
        // Note: In production, you'd get current price from market
        // For now, we'll use position's current_price
        self.position.update_pnl(self.position.current_price)?;

        // Check if removing margin would violate maintenance margin
        let new_margin = self.position.margin.checked_sub(amount).unwrap();
        let maintenance_margin_bps = 500; // 5% default
        if !meets_maintenance_margin(new_margin, self.position.pnl, maintenance_margin_bps) {
            return Err(PerpetualDexError::CannotRemoveMargin.into());
        }

        // Remove margin from position
        self.position.margin = new_margin;

        // Recalculate liquidation price
        self.position.liquidation_price = self.position.calculate_liquidation_price(
            self.position.entry_price,
            self.position.leverage,
            self.position.side,
        );

        // Free margin for user
        self.user.free_margin(amount, &self.clock)?;

        self.position.last_updated = self.clock.unix_timestamp;

        emit!(MarginRemoved {
            position_id: self.position.key(),
            user: self.trader.key(),
            amount,
            updated_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

