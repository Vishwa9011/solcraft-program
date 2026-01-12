use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

use crate::constants::FAUCET_CONFIG_SEEDS;
use crate::errors::FaucetError;
use crate::states::faucet::FaucetConfig;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(
      mut,
      seeds = [FAUCET_CONFIG_SEEDS.as_bytes()],
      bump = faucet_config.bump,
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
        associated_token::authority = depositor,
    )]
    pub depositor_ata: InterfaceAccount<'info, TokenAccount>,

    pub depositor: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let depositor_balance = ctx.accounts.depositor_ata.amount.clone();
    require!(depositor_balance >= amount, FaucetError::InsufficientFunds);

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.depositor_ata.to_account_info(),
                to: ctx.accounts.treasury_ata.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
