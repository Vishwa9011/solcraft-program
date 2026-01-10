use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FactoryConfig {
    pub admin: Pubkey,
    pub paused: bool,
    pub bump: u8,
    pub creation_fee_lamports: u64,
}
