use anchor_lang::prelude::*;

use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;
use anchor_spl::token_interface::{set_authority, Mint, SetAuthority, TokenInterface};

#[derive(Accounts)]
#[instruction(new_authority: Option<Pubkey>)]
pub struct TransferMintAuthority<'info> {
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub current_authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn transfer_mint_authority(
    ctx: Context<TransferMintAuthority>,
    new_authority: Option<Pubkey>,
) -> Result<()> {
    let set_authority_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.current_authority.to_account_info(),
        },
    );

    set_authority(
        set_authority_ctx,
        AuthorityType::MintTokens,
        new_authority.into(),
    )?;

    Ok(())
}
