use anchor_lang::prelude::*;
use crate::states::User;
use crate::events::CollateralDeposited;
use crate::errors::PerpetualDexError;

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = User::SIZE,
        seeds = [User::SEEDS, user.key().as_ref()],
        bump
    )]
    pub user_account: Box<Account<'info, User>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> DepositCollateral<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        require!(
            amount > 0,
            PerpetualDexError::InsufficientMargin
        );

        // Initialize user account if needed
        if self.user_account.user == Pubkey::default() {
            self.user_account.init(self.user.key(), &self.clock)?;
        }

        // In production, this would transfer tokens from user to the program
        // For now, we'll just update the account
        // The actual transfer would be handled via token program CPI

        self.user_account.deposit_collateral(amount, &self.clock)?;

        emit!(CollateralDeposited {
            user: self.user.key(),
            amount,
            deposited_at: self.clock.unix_timestamp,
        });

        Ok(())
    }
}

