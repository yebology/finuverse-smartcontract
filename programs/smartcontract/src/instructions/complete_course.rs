use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
#[instruction(course_id : u64)]
pub struct CompleteCourse<'info> {
    #[account(
        init, 
        payer=user,
        space=Rate::MAXIMUM_SIZE,
        seeds=[
            b"complete".as_ref(),
            user.key().as_ref(),
            &course_id.to_le_bytes()
        ],
        bump
    )]
    pub complete : Account<'info, Complete>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn complete_handler(
    ctx : Context<CompleteCourse>,
    course_id : u64,
    complete_answer : u64
) -> Result<()> {

    let complete = &mut ctx.accounts.complete;

    require!(
        (
            course_id > 0
        ),
        CourseError::InvalidCourseId
    );

    require!(
        (
            complete_answer >= 1 && complete_answer <= 3
        ),
        CourseError::InvalidCompleteAnswer
    );

    complete.course_id = course_id;
    complete.correct_answer = complete_answer;
    complete.user = *ctx.accounts.user.key;

    emit!(CompleteCreated {
        user: *ctx.accounts.user.key,
        complete: complete_answer,
        complete_account: complete.key()
    });

    Ok(())
}
