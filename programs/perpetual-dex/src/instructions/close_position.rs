use anchor_lang::prelude::*;
use crate::states::{GlobalConfig, Market, User, Position};
use crate::events::PositionClosed;
use crate::errors::PerpetualDexError;

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(
        seeds = [GlobalConfig::SEEDS],
        bump
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    #[account(
        mut,
        seeds = [Market::SEEDS, market.market_id.as_ref()],
        bump
    )]
    pub market: Box<Account<'info, Market>>,

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

impl<'info> ClosePosition<'info> {
    pub fn process(&mut self, _position_id: u64) -> Result<()> {
        // Update PnL with current price
        self.position.update_pnl(self.market.current_price)?;

        let pnl = self.position.pnl;
        let margin = self.position.margin;

        // Free margin
        self.user.free_margin(margin, &self.clock)?;

        // Update user PnL
        if pnl >= 0 {
            self.user.collateral = self.user.collateral.checked_add(pnl as u64).unwrap();
            self.user.free_margin = self.user.free_margin.checked_add(pnl as u64).unwrap();
        } else {
            let loss = (-pnl) as u64;
            self.user.collateral = self.user.collateral.checked_sub(loss).unwrap_or(0);
            self.user.free_margin = self.user.free_margin.checked_sub(loss).unwrap_or(0);
        }

        self.user.total_pnl = self.user.total_pnl.checked_add(pnl).unwrap();

        // Update market open interest
        let oi_delta = -(self.position.size as i64);
        match self.position.side {
            crate::consts::PositionSide::Long => {
                self.market.update_open_interest(oi_delta, 0)?;
            }
            crate::consts::PositionSide::Short => {
                self.market.update_open_interest(0, oi_delta)?;
            }
        }

        // Close position
        self.position.close(&self.clock)?;
        self.user.decrement_positions();

        emit!(PositionClosed {
            position_id: self.position.key(),
            user: self.trader.key(),
            market: self.market.key(),
            exit_price: self.market.current_price,
            pnl,
            closed_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

