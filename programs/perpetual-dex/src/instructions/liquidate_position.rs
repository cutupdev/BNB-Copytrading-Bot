use anchor_lang::prelude::*;
use crate::states::{GlobalConfig, Market, User, Position};
use crate::events::PositionLiquidated;
use crate::errors::PerpetualDexError;
use crate::utils::{should_liquidate, calculate_liquidation_bonus};

#[derive(Accounts)]
pub struct LiquidatePosition<'info> {
    #[account(
        seeds = [GlobalConfig::SEEDS],
        bump
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    #[account(
        seeds = [Market::SEEDS, market.market_id.as_ref()],
        bump
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        mut,
        seeds = [User::SEEDS, user.user.as_ref()],
        bump
    )]
    pub user: Box<Account<'info, User>>,

    #[account(
        mut,
        seeds = [Position::SEEDS, position.position_id.as_ref()],
        bump,
        constraint = !position.is_closed @ PerpetualDexError::PositionClosed
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> LiquidatePosition<'info> {
    pub fn process(&mut self, _position_id: u64) -> Result<()> {
        // Update PnL
        self.position.update_pnl(self.market.current_price)?;

        // Check if position is liquidatable
        require!(
            should_liquidate(
                self.market.current_price,
                self.position.liquidation_price,
                self.position.side,
            ),
            PerpetualDexError::PositionNotLiquidatable
        );

        // Calculate liquidation bonus
        let position_value = (self.position.size as u128)
            .checked_mul(self.market.current_price as u128)
            .unwrap()
            .checked_div(crate::consts::PRICE_PRECISION as u128)
            .unwrap() as u64;

        let liquidation_bonus = calculate_liquidation_bonus(
            position_value,
            self.global_config.liquidation_fee_bps,
        );

        // Free margin for user (what's left after liquidation)
        let remaining_margin = if self.position.pnl >= 0 {
            self.position.margin.checked_add(self.position.pnl as u64).unwrap_or(0)
        } else {
            self.position.margin.checked_sub((-self.position.pnl) as u64).unwrap_or(0)
        };

        if remaining_margin > liquidation_bonus {
            let user_remaining = remaining_margin.checked_sub(liquidation_bonus).unwrap();
            self.user.free_margin(user_remaining, &self.clock)?;
            self.user.collateral = self.user.collateral.checked_sub(self.position.margin).unwrap_or(0);
            self.user.collateral = self.user.collateral.checked_add(user_remaining).unwrap();
        } else {
            self.user.collateral = self.user.collateral.checked_sub(self.position.margin).unwrap_or(0);
        }

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

        // Note: In production, liquidation bonus would be transferred to liquidator
        // This would require additional accounts for token transfers

        emit!(PositionLiquidated {
            position_id: self.position.key(),
            user: self.user.user,
            market: self.market.key(),
            liquidation_price: self.market.current_price,
            pnl: self.position.pnl,
            liquidated_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

