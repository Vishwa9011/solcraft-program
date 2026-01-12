use anchor_lang::{prelude::*, system_program};

use crate::constants::*;
use crate::errors::FactoryError;
use crate::states::FactoryConfig;

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
