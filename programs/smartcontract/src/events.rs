use anchor_lang::prelude::*;

#[event]
pub struct RateCreated {
    pub user : Pubkey,
    pub rating : u64,
    pub course_account : Pubkey
}

#[event]
pub struct CompleteCreated {
    pub user : Pubkey,
    pub complete : u64,
    pub complete_account : Pubkey
}
