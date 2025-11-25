use anchor_lang::prelude::*;
use crate::states::{GlobalConfig, Market, User, Order};
use crate::consts::{OrderSide, OrderType};
use crate::events::OrderPlaced;
use crate::errors::PerpetualDexError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PlaceOrderArgs {
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: u64,
    pub price: u64,
    pub stop_price: Option<u64>,
    pub leverage: u16,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
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
        seeds = [User::SEEDS, user.user.as_ref()],
        bump,
        constraint = user.total_orders < crate::consts::MAX_OPEN_ORDERS as u64 @ PerpetualDexError::MaxOpenOrdersExceeded
    )]
    pub user: Box<Account<'info, User>>,

    /// CHECK: Order ID provided by user
    pub order_id: AccountInfo<'info>,

    #[account(
        init,
        payer = trader,
        space = Order::SIZE,
        seeds = [Order::SEEDS, order_id.key().as_ref()],
        bump
    )]
    pub order: Box<Account<'info, Order>>,

    #[account(mut)]
    pub trader: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> PlaceOrder<'info> {
    pub fn process(&mut self, args: PlaceOrderArgs) -> Result<()> {
        require!(
            args.leverage >= crate::consts::MIN_LEVERAGE_BPS 
                && args.leverage <= self.global_config.max_leverage_bps,
            PerpetualDexError::InvalidLeverage
        );

        // Calculate required margin
        let required_margin = crate::utils::calculate_required_margin(
            args.size,
            args.price,
            args.leverage,
        )?;

        require!(
            self.user.free_margin >= required_margin,
            PerpetualDexError::InsufficientMargin
        );

        // Reserve margin for order
        self.user.use_margin(required_margin, &self.clock)?;

        // Initialize order
        self.order.init(
            self.order_id.key(),
            self.trader.key(),
            self.market.key(),
            args.side,
            args.order_type,
            args.size,
            args.price,
            args.stop_price,
            args.leverage,
            &self.clock,
        )?;

        self.user.increment_orders();

        emit!(OrderPlaced {
            order_id: self.order.key(),
            user: self.trader.key(),
            market: self.market.key(),
            side: args.side,
            order_type: args.order_type,
            size: args.size,
            price: args.price,
            leverage: args.leverage,
            created_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

