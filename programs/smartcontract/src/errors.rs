use anchor_lang::prelude::*;

#[error_code]
pub enum FuniverseError {
    #[msg("Invalid course id.")]
    InvalidCourseId,
    #[msg("Invalid rating.")]
    InvalidRating
}