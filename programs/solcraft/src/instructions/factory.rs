use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::FactoryError;
use crate::states::FactoryConfig;

#[derive(Accounts)]
#[instruction(creation_fee_lamports: u64)]
pub struct InitializeFactory<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
      init_if_needed,
      payer = admin,
      space = DISCRIMINATOR as usize + FactoryConfig::INIT_SPACE,
      seeds = [FACTORY_CONFIG_SEEDS.as_bytes()],
      bump
    )]
    pub factory_config: Account<'info, FactoryConfig>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_factory(
    ctx: Context<InitializeFactory>,
    creation_fee_lamports: u64,
) -> Result<()> {
    let factory_config = &mut ctx.accounts.factory_config;
    factory_config.creation_fee_lamports = creation_fee_lamports;
    factory_config.admin = ctx.accounts.admin.key();
    factory_config.bump = ctx.bumps.factory_config;
    factory_config.paused = false;

    Ok(())
}

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

#[derive(Accounts)]
pub struct UnpauseFactory<'info> {
    #[account(
        mut,
        seeds = [FACTORY_CONFIG_SEEDS.as_bytes()],
        bump = factory_config.bump,
        has_one = admin @ FactoryError::Unauthorized,
    )]
    pub factory_config: Account<'info, FactoryConfig>,

    pub admin: Signer<'info>,
}

pub fn unpause_factory(ctx: Context<UnpauseFactory>) -> Result<()> {
    let factory_config = &mut ctx.accounts.factory_config;
    factory_config.paused = false;

    Ok(())
}
