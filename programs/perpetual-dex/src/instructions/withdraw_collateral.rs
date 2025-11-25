use anchor_lang::prelude::*;
use crate::states::User;
use crate::events::CollateralWithdrawn;
use crate::errors::PerpetualDexError;

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(
        mut,
        seeds = [User::SEEDS, user.key().as_ref()],
        bump,
        constraint = user_account.user == user.key() @ PerpetualDexError::Unauthorized
    )]
    pub user_account: Box<Account<'info, User>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> WithdrawCollateral<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        require!(
            amount > 0,
            PerpetualDexError::InsufficientMargin
        );

        // Withdraw collateral
        self.user_account.withdraw_collateral(amount, &self.clock)?;

        // In production, this would transfer tokens from program to user
        // The actual transfer would be handled via token program CPI

        emit!(CollateralWithdrawn {
            user: self.user.key(),
            amount,
            withdrawn_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

