use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
#[instruction(course_id : u64)]
pub struct RateCourse<'info> {
    #[account(
        init, 
        payer=user,
        space=Rate::MAXIMUM_SIZE,
        seeds=[
            b"rate".as_ref(),
            user.key().as_ref(),
            &course_id.to_le_bytes()
        ],
        bump
    )]
    pub rate : Account<'info, Rate>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn rate_handler(
    ctx : Context<RateCourse>,
    course_id : u64,
    rating : u64
) -> Result<()> {

    let rate = &mut ctx.accounts.rate;

    require!(
        (
            course_id > 0
        ),
        FuniverseError::InvalidCourseId
    );

    require!(
        (
            rating >= 1 && rating <= 5
        ),
        FuniverseError::InvalidRating
    );

    rate.course_id = course_id;
    rate.rating = rating;
    rate.user = *ctx.accounts.user.key;

    emit!(RateCreated {
        user: *ctx.accounts.user.key,
        rating: rating,
        course_account: rate.key()
    });

    Ok(())
}