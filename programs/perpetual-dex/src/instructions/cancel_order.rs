use anchor_lang::prelude::*;
use crate::states::{User, Order};
use crate::events::OrderCancelled;
use crate::errors::PerpetualDexError;

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(
        seeds = [User::SEEDS, user.user.as_ref()],
        bump,
        constraint = user.user == trader.key() @ PerpetualDexError::Unauthorized
    )]
    pub user: Box<Account<'info, User>>,

    #[account(
        mut,
        seeds = [Order::SEEDS, order.order_id.as_ref()],
        bump,
        constraint = order.user == trader.key() @ PerpetualDexError::Unauthorized
    )]
    pub order: Box<Account<'info, Order>>,

    #[account(mut)]
    pub trader: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> CancelOrder<'info> {
    pub fn process(&mut self, _order_id: u64) -> Result<()> {
        // Cancel order
        self.order.cancel(&self.clock)?;

        // Free up margin
        let required_margin = crate::utils::calculate_required_margin(
            self.order.size,
            self.order.price,
            self.order.leverage,
        )?;

        self.user.free_margin(required_margin, &self.clock)?;
        self.user.decrement_orders();

        emit!(OrderCancelled {
            order_id: self.order.key(),
            user: self.trader.key(),
            market: self.order.market,
            cancelled_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

