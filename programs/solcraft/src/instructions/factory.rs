use anchor_lang::{prelude::*, system_program};

use crate::constants::*;
use crate::errors::FactoryError;
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

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        seeds = [FACTORY_CONFIG_SEEDS.as_bytes()],
        bump = factory_config.bump,
        has_one = admin @ FactoryError::Unauthorized,
    )]
    pub factory_config: Account<'info, FactoryConfig>,

    #[account(
        mut,
        seeds = [FACTORY_TREASURY.as_bytes()],
        bump = factory_config.treasury_bump,
        address = factory_config.treasury_account,
    )]
    pub treasury_account: SystemAccount<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
    let treasury_info = ctx.accounts.treasury_account.to_account_info();
    let treasury_balance = treasury_info.lamports();

    // Calculate the amount that can be withdrawn while keeping the treasury account rent-exempt.
    let rent_exempt_minimum = Rent::get()?.minimum_balance(treasury_info.data_len());
    let withdrawable_amount = treasury_balance.saturating_sub(rent_exempt_minimum);

    if withdrawable_amount == 0 {
        return Ok(());
    }

    let seeds = &[
        FACTORY_TREASURY.as_bytes(),
        &[ctx.accounts.factory_config.treasury_bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.treasury_account.to_account_info(),
            to: ctx.accounts.admin.to_account_info(),
        },
        signer,
    );
    system_program::transfer(cpi_ctx, withdrawable_amount)?;

    Ok(())
}
