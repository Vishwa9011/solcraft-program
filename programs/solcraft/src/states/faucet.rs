use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FaucetConfig {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub allowed_claim_amount: u64,
    pub treasury_ata: Pubkey,
    pub cooldown_seconds: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct FaucetRecipientData {
    pub last_claimed_at: i64,
}
