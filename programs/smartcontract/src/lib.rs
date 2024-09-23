use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod state;
pub mod instructions;

use errors::*;
use events::*;
use state::*;
use instructions::*;

declare_id!("4DYP8c9XLTW88FrfPkJ1F1Ak4hcHLQy5kZXFo5QiJuAi");

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
    
}


