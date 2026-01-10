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
}
