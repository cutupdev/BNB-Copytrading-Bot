use anchor_lang::prelude::*;
use crate::states::{GlobalConfig, Market, User, Position};
use crate::consts::PositionSide;
use crate::events::PositionOpened;
use crate::errors::PerpetualDexError;
use crate::utils::calculate_required_margin;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OpenPositionArgs {
    pub side: PositionSide,
    pub size: u64,
    pub leverage: u16,
    pub margin: u64,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(
        seeds = [GlobalConfig::SEEDS],
        bump,
        constraint = !global_config.paused @ PerpetualDexError::MarketPaused
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    #[account(
        seeds = [Market::SEEDS, market.market_id.as_ref()],
        bump,
        constraint = !market.paused @ PerpetualDexError::MarketPaused
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(
        mut,
        seeds = [User::SEEDS, user.user.as_ref()],
        bump
    )]
    pub user: Box<Account<'info, User>>,

    /// CHECK: Position ID provided by user
    pub position_id: AccountInfo<'info>,

    #[account(
        init,
        payer = trader,
        space = Position::SIZE,
        seeds = [Position::SEEDS, position_id.key().as_ref()],
        bump
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(mut)]
    pub trader: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> OpenPosition<'info> {
    pub fn process(&mut self, args: OpenPositionArgs) -> Result<()> {
        require!(
            args.leverage >= crate::consts::MIN_LEVERAGE_BPS 
                && args.leverage <= self.global_config.max_leverage_bps,
            PerpetualDexError::InvalidLeverage
        );

        require!(
            args.size <= crate::consts::MAX_POSITION_SIZE,
            PerpetualDexError::MaxPositionSizeExceeded
        );

        // Calculate required margin
        let required_margin = calculate_required_margin(
            args.size,
            self.market.current_price,
            args.leverage,
        )?;

        require!(
            args.margin >= required_margin,
            PerpetualDexError::InsufficientMargin
        );

        require!(
            self.user.free_margin >= args.margin,
            PerpetualDexError::InsufficientMargin
        );

        // Use margin
        self.user.use_margin(args.margin, &self.clock)?;

        // Initialize position
        self.position.init(
            self.position_id.key(),
            self.trader.key(),
            self.market.key(),
            args.side,
            args.size,
            self.market.current_price,
            args.leverage,
            args.margin,
            &self.clock,
        )?;

        // Update market open interest
        let oi_delta = args.size as i64;
        match args.side {
            PositionSide::Long => {
                self.market.update_open_interest(oi_delta, 0)?;
            }
            PositionSide::Short => {
                self.market.update_open_interest(0, oi_delta)?;
            }
        }

        self.user.increment_positions();

        emit!(PositionOpened {
            position_id: self.position.key(),
            user: self.trader.key(),
            market: self.market.key(),
            side: args.side,
            size: args.size,
            entry_price: self.market.current_price,
            leverage: args.leverage,
            margin: args.margin,
            opened_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

