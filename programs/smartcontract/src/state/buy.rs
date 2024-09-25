use anchor_lang::prelude::*;

#[account]
pub struct Buy {
    pub course_id: u64,
    pub buyer: Pubkey,
}

impl Buy {
    const DISCRIMINATOR_SPACE: usize = 8;
    const ID_SPACE: usize = 8;
    const BUYER_SPACE: usize = 32;

    pub const MAXIMUM_SIZE: usize = Self::DISCRIMINATOR_SPACE + Self::ID_SPACE + Self::BUYER_SPACE;
}