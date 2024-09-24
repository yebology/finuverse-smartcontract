use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::*;

#[derive(Accounts)]
pub struct BuyCourse<'info> {
    #[account(
        mut
    )]
    pub course: Account<'info, Course>, 
    #[account(mut)]
    pub buyer: Signer<'info>, 
    pub system_program: Program<'info, System>
}

pub fn buy_handler(
    ctx: Context<BuyCourse>,
    course_id: u64, 
) -> Result<()> {
    let course = &mut ctx.accounts.course; 
    let buyer = &ctx.accounts.buyer; 

    require!(buyer.lamports() >= course.price, CourseError::InsufficientFunds);

    let cpi_account = system_program::Transfer {
        from: buyer.to_account_info(),
        to: course.creator.to_account_info(), 
    };

    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_account);
    system_program::transfer(cpi_context, course.price)?;

    course.buyer += 1; 

    emit!(CoursePurchased {
        buyer: buyer.key(), 
        course_id: course.id,
    });

    Ok(())
}