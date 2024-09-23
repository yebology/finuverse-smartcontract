use anchor_lang::prelude::*;

#[account]
pub struct Rate {
    pub user : Pubkey,
    pub course_id : u64,
    pub rating : u64
}

impl Rate {
    const DISCRIMINATOR_SPACE : usize = 8;
    const USER_SPACE : usize = 32;
    const COURSE_ID_SPACE : usize = 8;
    const RATING_SPACE : usize = 8;

    pub const MAXIMUM_SIZE : usize = 
    Self::DISCRIMINATOR_SPACE +
    Self::USER_SPACE + 
    Self::COURSE_ID_SPACE + 
    Self::RATING_SPACE;
}