use anchor_lang::prelude::*;

//NOTE: if we don't need the constant in client side, we can remove the #[constant] attribute

// Discriminator size for accounts
pub const DISCRIMINATOR: usize = 8;

#[constant]
pub const FACTORY_CONFIG_SEEDS: &str = "factory_config";

#[constant]
pub const FACTORY_TREASURY: &str = "factory_treasury";

#[constant]
pub const MAX_DECIMALS: u8 = 9;
