use anchor_lang::prelude::*;

#[event]
pub struct CoursePurchased {
    pub buyer: Pubkey,
    pub course_id: u64,
}