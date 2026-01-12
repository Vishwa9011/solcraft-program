use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::{DISCRIMINATOR, FAUCET_CONFIG_SEEDS, MAX_DECIMALS};
use crate::errors::FaucetError;
use crate::states::FaucetConfig;

#[derive(Accounts)]
pub struct InitializeFaucet<'info> {
    #[account(
      init,
      payer = owner,
      space = DISCRIMINATOR + FaucetConfig::INIT_SPACE,
      seeds = [FAUCET_CONFIG_SEEDS.as_bytes()],
      bump,
   )]
    pub faucet_config: Account<'info, FaucetConfig>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
      init,
      payer = owner,
      associated_token::mint = mint,
      associated_token::authority = faucet_config,
    )]
    pub treasury_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<InitializeFaucet>) -> Result<()> {
    let faucet_config = &mut ctx.accounts.faucet_config;
    let decimals = ctx.accounts.mint.decimals;
    require!(decimals <= MAX_DECIMALS, FaucetError::ExceedsMaxDecimals);

    faucet_config.owner = ctx.accounts.owner.key();
    faucet_config.mint = ctx.accounts.mint.key();
    faucet_config.treasury_ata = ctx.accounts.treasury_ata.key();
    faucet_config.cooldown_seconds = 3600; // default 1 hour
    faucet_config.allowed_claim_amount = 1000 * 10u64.pow(decimals as u32); // default 1000 tokens
    faucet_config.bump = ctx.bumps.faucet_config;

    Ok(())
}
