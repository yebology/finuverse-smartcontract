use anchor_lang::prelude::*;

#[error_code]
pub enum CourseError {
    #[msg("Insufficient funds for the purchase.")]
    InsufficientFunds,
    #[msg("Invalid Course Input")]
    InvalidCourseInput,
    #[msg("Invalid course id.")]
    InvalidCourseId,
    #[msg("Invalid rating.")]
    InvalidRating,
    #[msg("Invalid complete answer")]
    InvalidCompleteAnswer
}