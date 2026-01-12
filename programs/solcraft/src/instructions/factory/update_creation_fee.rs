use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::FactoryError;
use crate::states::FactoryConfig;

#[derive(Accounts)]
#[instruction(creation_fee_lamports: u64)]
pub struct UpdateCreationFee<'info> {
    #[account(
        mut,
        seeds = [FACTORY_CONFIG_SEEDS.as_bytes()],
        bump = factory_config.bump,
        has_one = admin @ FactoryError::Unauthorized,
    )]
    pub factory_config: Account<'info, FactoryConfig>,

    pub admin: Signer<'info>,
}

pub fn update_creation_fee(
    ctx: Context<UpdateCreationFee>,
    creation_fee_lamports: u64,
) -> Result<()> {
    let factory_config = &mut ctx.accounts.factory_config;
    factory_config.creation_fee_lamports = creation_fee_lamports;

    Ok(())
}
