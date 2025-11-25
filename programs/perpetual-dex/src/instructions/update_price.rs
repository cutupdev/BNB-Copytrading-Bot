use anchor_lang::prelude::*;
use crate::states::Market;
use crate::events::PriceUpdated;
use crate::errors::PerpetualDexError;

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(
        mut,
        seeds = [Market::SEEDS, market.market_id.as_ref()],
        bump
    )]
    pub market: Box<Account<'info, Market>>,

    /// CHECK: Oracle or authorized price updater
    pub oracle: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> UpdatePrice<'info> {
    pub fn process(&mut self, price: u64) -> Result<()> {
        require!(
            price > 0,
            PerpetualDexError::InvalidPrice
        );

        let old_price = self.market.current_price;

        // Update price
        self.market.update_price(price, &self.clock)?;

        emit!(PriceUpdated {
            market: self.market.key(),
            old_price,
            new_price: price,
            updated_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

