use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Incorrect voting power for burning: {0}")]
    IncorrectBalance(Uint128),

    #[error("Wallet not found")]
    NotFound {},
}
