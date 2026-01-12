use anchor_lang::{prelude::*, system_program};

use crate::constants::*;
use crate::states::FactoryConfig;

#[derive(Accounts)]
#[instruction(creation_fee_lamports: u64)]
pub struct InitializeFactory<'info> {
    #[account(
        init,
        payer = admin,
        space = DISCRIMINATOR + FactoryConfig::INIT_SPACE,
        seeds = [FACTORY_CONFIG_SEEDS.as_bytes()],
        bump
    )]
    pub factory_config: Account<'info, FactoryConfig>,

    #[account(
        init,
        payer = admin,
        space = 0,
        seeds = [FACTORY_TREASURY.as_bytes()],
        bump,
        owner = system_program::ID
    )]
    /// CHECK: This is a system account to hold lamports as treasury
    pub treasury_account: UncheckedAccount<'info>,

    // Admin pays for account creation; must be mutable to debit lamports.
    #[account(mut)]
    pub admin: Signer<'info>,

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
    factory_config.treasury_account = ctx.accounts.treasury_account.key();
    factory_config.treasury_bump = ctx.bumps.treasury_account;

    Ok(())
}
