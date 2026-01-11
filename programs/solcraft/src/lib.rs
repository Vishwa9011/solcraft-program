use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod states;

use instructions::*;

declare_id!("CADbArgTHGSsSiMJfXdtGYjQeLRf55f6QoQW7bNphicC");

#[program]
pub mod solcraft {
    use super::*;

    pub fn initialize_factory(
        ctx: Context<InitializeFactory>,
        creation_fee_lamports: u64,
    ) -> Result<()> {
        instructions::factory::initialize_factory(ctx, creation_fee_lamports)
    }

    pub fn update_creation_fee(
        ctx: Context<UpdateCreationFee>,
        creation_fee_lamports: u64,
    ) -> Result<()> {
        instructions::factory::update_creation_fee(ctx, creation_fee_lamports)
    }

    pub fn pause_factory(ctx: Context<PauseFactory>) -> Result<()> {
        instructions::factory::pause_factory(ctx)
    }

    pub fn unpause_factory(ctx: Context<UnpauseFactory>) -> Result<()> {
        instructions::factory::unpause_factory(ctx)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        instructions::factory::withdraw_fees(ctx)
    }

    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        uri: String,
        decimals: u8,
        supply: u64,
    ) -> Result<()> {
        instructions::token::create_token(ctx, name, symbol, uri, decimals, supply)
    }
}
