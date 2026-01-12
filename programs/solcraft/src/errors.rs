use anchor_lang::prelude::*;

#[error_code]
pub enum TokenError {
    #[msg("The provided decimals exceed the maximum allowed.")]
    ExceedsMaxDecimals,

    #[msg("The provided string length is invalid.")]
    InvalidInputStringLength,
}

#[error_code]
pub enum FactoryError {
    #[msg("The factory is currently paused.")]
    FactoryPaused,

    #[msg("Insufficient funds to cover the creation fee.")]
    InsufficientCreationFee,

    #[msg("Unauthorized action attempted.")]
    Unauthorized,

    #[msg("No funds available to withdraw.")]
    InsufficientFundsToWithdraw,
}

#[error_code]
pub enum FaucetError {
    #[msg("Insufficient funds in the depositor's account.")]
    InsufficientFunds,

    #[msg("Cooldown period has not yet elapsed.")]
    CooldownNotElapsed,

    #[msg("Unauthorized action attempted.")]
    Unauthorized,

    #[msg("The provided treasury ATA does not match the faucet configuration.")]
    InvalidTreasuryAta,

    #[msg("The provided decimals exceed the maximum allowed.")]
    ExceedsMaxDecimals,
}
