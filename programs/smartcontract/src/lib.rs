use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod state;
pub mod instructions;

use errors::*;
use events::*;
use state::*;
use instructions::*;

declare_id!("GHTaLRLyapNJumLVbyLpUYPesV9WiQBaVKTMKuawZtLh");

#[program]
pub mod smartcontract {
    use super::*;

    pub fn rate_course(
        ctx: Context<RateCourse>,
        course_id: u64,
        rating: u64
    ) -> Result<()> {
        instructions::rate_course::rate_handler(
            ctx,
            course_id,
            rating
        )?;
        Ok(())
    }
    
    pub fn complete_course(
        ctx: Context<CompleteCourse>,
        course_id: u64,
        correct_answer: u64
    ) -> Result<()> {
        instructions::complete_course::complete_handler(
            ctx,
            course_id,
            correct_answer
        )?;
        Ok(())
    }
    
}


