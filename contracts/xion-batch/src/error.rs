use cosmwasm_std::{CheckedFromRatioError, ConversionOverflowError, OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    CheckedFromRatioError(#[from] CheckedFromRatioError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("The reply ID is unrecognized")]
    UnrecognizedReply {},

    #[error("Invalid denom! Got: {got} - Expected: {expected}")]
    InvalidDenom { got: String, expected: String },

    #[error("Invalid funds! Got: {got} - Expected: {expected}")]
    InvalidFunds { got: String, expected: String },

    #[error("Insufficient funds!")]
    InsufficientFunds {},

    #[error("No funds found!")]
    MissingFunds {},

    #[error("Extra funds found! Only one coin is allowed to be sent to this message.")]
    ExtraFunds {},

    #[error("Invalid decimal")]
    InvalidDecimal {},
}
