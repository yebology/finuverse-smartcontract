use anchor_lang::prelude::*;

#[event]
pub struct CoursePurchased {
    pub buyer: Pubkey,
    pub course_id: u64,
}

#[event]
pub struct CreateCoursed{
    pub creator: Pubkey,
    pub course_account: Pubkey,
}