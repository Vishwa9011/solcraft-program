use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

#[derive(Accounts)]
#[instruction(amount:u64)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub recipient: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let mint_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.recipient_ata.to_account_info(),
        authority: ctx.accounts.recipient.to_account_info(),
    };

    let mint_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_accounts);

    mint_to(mint_ctx, amount)?;

    Ok(())
}
