use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use errors::*;
use events::*;
use instructions::*;
use state::*;

declare_id!("J2DZcUjM84mvzSm8HCoPFx58JouvJ6bogjRPdaeewghx");
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

    pub fn buy_course(ctx: Context<BuyCourse>, course_id: u64) -> Result<()> {
        instructions::buy_course::buy_handler(ctx, course_id)?;
        Ok(())
    }

    pub fn create_course(
        ctx: Context<CreateCourse>,
        id: u64,
        name: String,
        description: String,
        price: u64,
        thumbnail: String,
        section_title: [String; 3],
        section_description: [String; 3],
        section_duration: [u64; 3],
        section_video: [String; 3],
        question_list: [String; 3],
        answer_list: [String; 3],
        first_answer_options: [String; 4],
        second_answer_options: [String; 4],
        third_answer_options: [String; 4],
    ) -> Result<()> {
        instructions::create_course::create_handler(
            ctx,
            id,
            name,
            description,
            price,
            thumbnail,
            section_title,
            section_description,
            section_duration,
            section_video,
            question_list,
            answer_list,
            first_answer_options,
            second_answer_options,
            third_answer_options,
        )?;
        Ok(())
    }
}