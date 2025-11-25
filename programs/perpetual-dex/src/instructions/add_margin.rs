use anchor_lang::prelude::*;
use crate::states::{User, Position};
use crate::events::MarginAdded;
use crate::errors::PerpetualDexError;

#[derive(Accounts)]
pub struct AddMargin<'info> {
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

impl<'info> AddMargin<'info> {
    pub fn process(&mut self, _position_id: u64, amount: u64) -> Result<()> {
        require!(
            amount > 0,
            PerpetualDexError::InsufficientMargin
        );

        require!(
            self.user.free_margin >= amount,
            PerpetualDexError::InsufficientMargin
        );

        // Use margin
        self.user.use_margin(amount, &self.clock)?;

        // Add to position margin
        self.position.margin = self.position.margin.checked_add(amount).unwrap();

        // Recalculate liquidation price with new margin
        self.position.liquidation_price = self.position.calculate_liquidation_price(
            self.position.entry_price,
            self.position.leverage,
            self.position.side,
        );

        self.position.last_updated = self.clock.unix_timestamp;

        emit!(MarginAdded {
            position_id: self.position.key(),
            user: self.trader.key(),
            amount,
            updated_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

