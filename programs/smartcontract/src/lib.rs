use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use errors::*;
use events::*;
use instructions::*;
use state::*;

declare_id!("4DYP8c9XLTW88FrfPkJ1F1Ak4hcHLQy5kZXFo5QiJuAi");

#[program]
pub mod smartcontract {
    use super::*;

    pub fn buy_course(
        ctx: Context<BuyCourse>,
        course_id: u64
        ) -> Result<()> {
        instructions::buy_course::buy_handler(
            ctx,
            course_id
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}