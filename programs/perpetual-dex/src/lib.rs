use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use crate::instructions::*;

declare_id!("PERP1111111111111111111111111111111111111111");

#[program]
pub mod perpetual_dex {

    use super::*;

    /// Initialize the global configuration
    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        ctx.accounts.process(params)
    }

    /// Create a new perpetual market
    pub fn create_market(ctx: Context<CreateMarket>, args: CreateMarketArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// Place an order in the orderbook
    pub fn place_order(ctx: Context<PlaceOrder>, args: PlaceOrderArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// Cancel an existing order
    pub fn cancel_order(ctx: Context<CancelOrder>, order_id: u64) -> Result<()> {
        ctx.accounts.process(order_id)
    }

    /// Open a perpetual position (long or short)
    pub fn open_position(ctx: Context<OpenPosition>, args: OpenPositionArgs) -> Result<()> {
        ctx.accounts.process(args)
    }

    /// Close a perpetual position
    pub fn close_position(ctx: Context<ClosePosition>, position_id: u64) -> Result<()> {
        ctx.accounts.process(position_id)
    }

    /// Add margin to an existing position
    pub fn add_margin(ctx: Context<AddMargin>, position_id: u64, amount: u64) -> Result<()> {
        ctx.accounts.process(position_id, amount)
    }

    /// Remove margin from a position
    pub fn remove_margin(ctx: Context<RemoveMargin>, position_id: u64, amount: u64) -> Result<()> {
        ctx.accounts.process(position_id, amount)
    }

    /// Liquidate an undercollateralized position
    pub fn liquidate_position(ctx: Context<LiquidatePosition>, position_id: u64) -> Result<()> {
        ctx.accounts.process(position_id)
    }

    /// Deposit collateral to user account
    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    /// Withdraw collateral from user account
    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    /// Update market price (oracle)
    pub fn update_price(ctx: Context<UpdatePrice>, price: u64) -> Result<()> {
        ctx.accounts.process(price)
    }
}

