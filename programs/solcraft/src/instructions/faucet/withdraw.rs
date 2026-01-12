use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

use crate::constants::FAUCET_CONFIG_SEEDS;
use crate::errors::FaucetError;
use crate::states::faucet::FaucetConfig;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    #[account(
      mut,
      seeds = [FAUCET_CONFIG_SEEDS.as_bytes()],
      bump = faucet_config.bump,
      constraint = recipient.key() == faucet_config.owner @ FaucetError::Unauthorized,
   )]
    pub faucet_config: Account<'info, FaucetConfig>,

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

    pub recipient: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let treasury_balance = ctx.accounts.treasury_ata.amount.clone();
    require!(treasury_balance >= amount, FaucetError::InsufficientFunds);

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
        amount,
    )?;

    Ok(())
}
