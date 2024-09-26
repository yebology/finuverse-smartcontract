use crate::*;
use anchor_lang::system_program;

#[derive(Accounts)]
#[instruction(course_id : u64)]
pub struct BuyCourse<'info> {
    #[account(
        init,
        payer=buyer,
        space=Buy::MAXIMUM_SIZE,
        seeds=[
            b"buy".as_ref(), 
            buyer.key().as_ref(),
            &course_id.to_le_bytes()
        ],
        bump
    )]
    pub buy: Account<'info, Buy>,
    #[account(mut)]
    pub course: Account<'info, Course>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: `to` is used as a destination for transferring funds.
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn buy_handler(ctx: Context<BuyCourse>, course_id: u64) -> Result<()> {
    let course = &mut ctx.accounts.course;
    let buyer = &ctx.accounts.buyer;
    let buy_course = &mut ctx.accounts.buy;

    require!(
        buyer.lamports() >= course.price,
        CourseError::InsufficientFunds
    );

    let cpi_account = system_program::Transfer {
        from: buyer.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_account);
    system_program::transfer(cpi_context, course.price)?;

    course.buyer += 1;

    buy_course.course_id = course_id;
    buy_course.buyer = buyer.key();

    emit!(CoursePurchased {
        buyer: buyer.key(),
        course_id: course.id,
    });

    Ok(())
}