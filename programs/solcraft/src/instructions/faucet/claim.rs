use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

use crate::constants::{DISCRIMINATOR, FAUCET_CONFIG_SEEDS, FAUCET_RECIPIENT_SEEDS};
use crate::errors::FaucetError;
use crate::states::faucet::{FaucetConfig, FaucetRecipientData};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(
      mut,
      seeds = [FAUCET_CONFIG_SEEDS.as_bytes()],
      bump = faucet_config.bump,
   )]
    pub faucet_config: Account<'info, FaucetConfig>,

    #[account(
      init_if_needed,
      payer = recipient,
      space = DISCRIMINATOR + FaucetRecipientData::INIT_SPACE,
      seeds = [FAUCET_RECIPIENT_SEEDS.as_bytes(), recipient.key().as_ref()],
      bump,
    )]
    pub recipient_data: Account<'info, FaucetRecipientData>,

    #[account(
        mut,
        associated_token::mint = faucet_config.mint,
        associated_token::authority = faucet_config,
        constraint = faucet_config.treasury_ata == treasury_ata.key() @ FaucetError::InvalidTreasuryAta,
    )]
    pub treasury_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = faucet_config.mint,
        associated_token::authority = recipient,
    )]
    pub recipient_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub recipient: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let faucet_config = &ctx.accounts.faucet_config;
    let claim_amount = ctx.accounts.faucet_config.allowed_claim_amount;
    let treasury_ata_info = ctx.accounts.treasury_ata.amount.clone();
    require!(
        treasury_ata_info >= claim_amount,
        FaucetError::InsufficientFunds
    );

    let last_claimed_at = ctx.accounts.recipient_data.last_claimed_at;

    if last_claimed_at != 0 {
        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(
            current_timestamp - last_claimed_at >= faucet_config.cooldown_seconds as i64,
            FaucetError::CooldownNotElapsed
        );
    }

    let seeds = &[
        FAUCET_CONFIG_SEEDS.as_bytes(),
        &[ctx.accounts.faucet_config.bump],
    ];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury_ata.to_account_info(),
                to: ctx.accounts.recipient_ata.to_account_info(),
                authority: ctx.accounts.faucet_config.to_account_info(),
            },
            &[seeds],
        ),
        claim_amount,
    )?;

    ctx.accounts.recipient_data.last_claimed_at = Clock::get()?.unix_timestamp;

    Ok(())
}
