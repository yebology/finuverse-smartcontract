use anchor_lang::prelude::*;

#[event]
pub struct RateCreated {
    pub user : Pubkey,
    pub rating : u64,
    pub course_account : Pubkey
}