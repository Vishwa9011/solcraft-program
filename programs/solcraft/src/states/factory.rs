use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FactoryConfig {
    pub admin: Pubkey,
    pub paused: bool,
    pub treasury_account: Pubkey,
    pub bump: u8,
    pub treasury_bump: u8,
    pub creation_fee_lamports: u64,
}
