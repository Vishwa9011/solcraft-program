use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::FactoryError;
use crate::states::FactoryConfig;

#[derive(Accounts)]
pub struct PauseFactory<'info> {
    #[account(
        mut,
        seeds = [FACTORY_CONFIG_SEEDS.as_bytes()],
        bump = factory_config.bump,
        has_one = admin @ FactoryError::Unauthorized,
    )]
    pub factory_config: Account<'info, FactoryConfig>,

    pub admin: Signer<'info>,
}

pub fn pause_factory(ctx: Context<PauseFactory>) -> Result<()> {
    let factory_config = &mut ctx.accounts.factory_config;
    factory_config.paused = true;

    Ok(())
}
