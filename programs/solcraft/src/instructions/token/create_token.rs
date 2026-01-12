use anchor_lang::prelude::*;
use anchor_lang::system_program;

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::{CollectionDetails, Creator, DataV2},
        sign_metadata, CreateMetadataAccountsV3, Metadata, SignMetadata,
    },
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use crate::constants::{FACTORY_CONFIG_SEEDS, FACTORY_TREASURY, MAX_DECIMALS};
use crate::errors::{FactoryError, TokenError};
use crate::states::FactoryConfig;

#[derive(Accounts)]
#[instruction(name:String, symbol:String, uri:String, decimals:u8, supply:u64)]
pub struct CreateToken<'info> {
    #[account(
        seeds = [FACTORY_CONFIG_SEEDS.as_bytes()],
        bump = factory_config.bump,
        constraint = !factory_config.paused @ FactoryError::FactoryPaused,
    )]
    pub factory_config: Account<'info, FactoryConfig>,

    #[account(
        mut,
        seeds = [FACTORY_TREASURY.as_bytes()],
        bump,
        address = factory_config.treasury_account,
    )]
    pub treasury_account: SystemAccount<'info>,

    #[account(
         init,
         payer= payer,
         mint::authority = payer,
         mint::decimals = decimals,
         mint::freeze_authority = payer,
      )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub payer_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: This account is checked by metadata smart contract
    pub metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_token(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    uri: String,
    decimals: u8,
    supply: u64,
) -> Result<()> {
    require!(name.len() <= 32, TokenError::InvalidInputStringLength);
    require!(decimals <= MAX_DECIMALS, TokenError::ExceedsMaxDecimals);
    require!(symbol.len() <= 10, TokenError::InvalidInputStringLength);
    require!(uri.len() <= 200, TokenError::InvalidInputStringLength);

    // check the balance of payer to ensure they can pay creation fee
    let payer_lamports = ctx.accounts.payer.to_account_info().lamports();
    require!(
        payer_lamports >= ctx.accounts.factory_config.creation_fee_lamports,
        FactoryError::InsufficientCreationFee
    );

    // Mint to payer's associated token account
    let mint_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.payer_ata.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    // mint_ctx
    let mint_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_accounts);

    // mint tokens to payer's associated token account
    mint_to(mint_ctx, supply)?;

    // Now Create metadata for mint using metaplex token metadata program
    let metadata_accounts = CreateMetadataAccountsV3 {
        metadata: ctx.accounts.metadata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        mint_authority: ctx.accounts.payer.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        update_authority: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };

    let metadata_ctx = CpiContext::new(
        ctx.accounts.token_metadata_program.to_account_info(),
        metadata_accounts,
    );

    let data_v2 = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        creators: Some(vec![Creator {
            address: ctx.accounts.payer.key(),
            verified: false, // will be set to true by the token metadata program
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    create_metadata_accounts_v3(
        metadata_ctx,
        data_v2,
        true,
        true,
        Some(CollectionDetails::V1 { size: 0 }),
    )?;

    sign_metadata(CpiContext::new(
        ctx.accounts.token_metadata_program.to_account_info(),
        SignMetadata {
            creator: ctx.accounts.payer.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
        },
    ))?;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.treasury_account.to_account_info(),
            },
        ),
        ctx.accounts.factory_config.creation_fee_lamports,
    )?;

    Ok(())
}
