use anchor_lang::prelude::*;
use crate::states::{GlobalConfig, Market};
use crate::events::MarketCreated;
use crate::errors::PerpetualDexError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateMarketArgs {
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub initial_price: u64,
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(
        seeds = [GlobalConfig::SEEDS],
        bump,
        constraint = !global_config.paused @ PerpetualDexError::MarketPaused
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    /// CHECK: Market ID provided by user
    pub market_id: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Market::BASE_SIZE + 
            4 + 50 + // name (max 50)
            4 + 10,  // symbol (max 10)
        seeds = [Market::SEEDS, market_id.key().as_ref()],
        bump
    )]
    pub market: Box<Account<'info, Market>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> CreateMarket<'info> {
    pub fn process(&mut self, args: CreateMarketArgs) -> Result<()> {
        require!(
            args.initial_price > 0,
            PerpetualDexError::InvalidPrice
        );

        self.market.init(
            self.market_id.key(),
            args.base_mint,
            args.quote_mint,
            args.name.clone(),
            args.symbol.clone(),
            args.initial_price,
            &self.clock,
        )?;

        self.global_config.increment_total_markets();

        emit!(MarketCreated {
            market: self.market.key(),
            base_mint: args.base_mint,
            quote_mint: args.quote_mint,
            name: args.name,
            created_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

