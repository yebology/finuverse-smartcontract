use anchor_lang::prelude::*;

#[error_code]
pub enum FinuverseError {
    #[msg("Insufficient funds for the purchase.")]
    InsufficientFunds,
    #[msg("Invalid Course Input")]
    InvalidCourseInput,
}
