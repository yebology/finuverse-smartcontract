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
    pub fn create_course(
        ctx: Context<CreateCourse>,
        id: u64,
        name: String,
        creator: Pubkey,
        description: String,
        price: u64,
        buyer: u64,
        thumbnail: u64,
        section_title: [String;3],
        section_description:[String;3],
        section_duration:[u64;3],
        section_video:[String;3],
        question_list:[String;3],
        answer_list:[String;3],
        first_answer_options:[String;4],
        second_answer_options:[String;4],
        third_answer_options:[String;4]
    )-> Result<()>{
        instruction::create_course::create_handler(
            ctx,
            id,
            name,
            creator,
            description,
            price,
            buyer,
            thumbnail,
            section_title,
            section_description,
            section_duration,
            section_video,
            question_list,
            answer_list,
            first_answer_options,
            second_answer_options,
            third_answer_options
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}