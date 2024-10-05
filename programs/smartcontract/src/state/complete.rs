use anchor_lang::prelude::*;

#[account]
pub struct Complete {
    pub user : Pubkey,
    pub course_id : u64,
    pub correct_answer : u64
}

impl Complete {
    const DISCRIMINATOR_SPACE : usize = 8;
    const USER_SPACE : usize = 32;
    const COURSE_ID_SPACE : usize = 8;
    const CORRECT_ANSWER_SPACE : usize = 8;

    pub const MAXIMUM_SIZE : usize = 
    Self::DISCRIMINATOR_SPACE +
    Self::USER_SPACE + 
    Self::COURSE_ID_SPACE + 
    Self::CORRECT_ANSWER_SPACE;
}
