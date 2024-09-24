use anchor_lang::prelude::*;

#[error_code]
pub enum CourseError {
    #[msg("Insufficient funds for the purchase.")]
    InsufficientFunds,
}