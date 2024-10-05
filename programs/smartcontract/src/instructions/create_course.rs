use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateCourse<'info> {
    #[account(
        init,
        payer=creator,
        space=Course::MAXIMUM_SIZE,
        seeds=[
            b"course".as_ref(),
            creator.key.as_ref(),
            &id.to_le_bytes()
        ],
        bump
    )]
    pub course: Account<'info, Course>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_handler(
    ctx: Context<CreateCourse>,
    id: u64,
    name: String,
    description: String,
    category: u64,
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
    let course = &mut ctx.accounts.course;

    require!(
        (id > 0
            && !name.is_empty()
            && !description.is_empty()
            && (category == 1 || category == 2 || category == 3)
            && price > 0
            && !thumbnail.is_empty()
            && section_title.len() == 3
            && section_description.len() == 3
            && section_duration.len() == 3
            && section_video.len() == 3
            && question_list.len() == 3
            && answer_list.len() == 3
            && first_answer_options.len() == 4
            && second_answer_options.len() == 4
            && third_answer_options.len() == 4),
        CourseError::InvalidCourseInput
    );
    course.id = id;
    course.name = name;
    course.creator = *ctx.accounts.creator.key;
    course.description = description;
    course.category = category;
    course.price = price;
    course.thumbnail = thumbnail;
    course.buyer = 0;
    course.section_title = section_title;
    course.section_description = section_description;
    course.section_duration = section_duration;
    course.section_video = section_video;
    course.question_list = question_list;
    course.answer_list = answer_list;
    course.first_answer_options = first_answer_options;
    course.second_answer_options = second_answer_options;
    course.third_answer_options = third_answer_options;
    emit!(CreateCoursed {
        creator: *ctx.accounts.creator.key,
        course_account: course.key()
    });
    Ok(())
}