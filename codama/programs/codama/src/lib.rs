use anchor_lang::prelude::*;

declare_id!("4sjhiC2Yq1uQmt8Hj2r6zJ2z6uRPf5151g68sBDKmZHZ");

#[program]
pub mod codama {
    use super::*;

    pub fn set_review(ctx: Context<Restaurant>, name: String, stars: u8) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Review for {} with {} stars", name, stars);
        
        // Store the data
        let restaurant = &mut ctx.accounts.restaurant_review;
        restaurant.restaurant_name = name;
        restaurant.restaurant_stars = stars;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(restaurant_name: String)]
pub struct Restaurant<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + ResturantDetails::INIT_SPACE, seeds= [b"reviews", user.key().as_ref()], bump)]
    pub restaurant_review: Account<'info, ResturantDetails>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct ResturantDetails {
    #[max_len(50)]
    pub restaurant_name: String,
    pub restaurant_stars: u8,
}
