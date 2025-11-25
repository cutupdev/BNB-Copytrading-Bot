use anchor_lang::prelude::*;
use crate::states::GlobalConfig;
use crate::errors::PerpetualDexError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeParams {
    pub fee_recipient: Pubkey,
    pub trading_fee_bps: u16,
    pub liquidation_fee_bps: u16,
    pub max_leverage_bps: u16,
    pub initial_margin_bps: u16,
    pub maintenance_margin_bps: u16,
    pub liquidation_threshold_bps: u16,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GlobalConfig::SIZE,
        seeds = [GlobalConfig::SEEDS],
        bump
    )]
    pub global_config: Box<Account<'info, GlobalConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(&self, params: InitializeParams) -> Result<()> {
        self.global_config.init(
            self.authority.key(),
            params.fee_recipient,
            params.trading_fee_bps,
            params.liquidation_fee_bps,
            params.max_leverage_bps,
            params.initial_margin_bps,
            params.maintenance_margin_bps,
            params.liquidation_threshold_bps,
        )?;

        Ok(())
    }
}

