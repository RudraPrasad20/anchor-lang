use anchor_lang::prelude::*;

declare_id!("CFqCu1qy3dD1iJvtAo4wsrrzpfoKUivjacKsHwBpvC82");

pub const ANCHOR_DESCRIMINATOR_SIZE: usize = 8; 

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourates(ctx: Context<SetFavourites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let user_public_key = ctx.accounts.user.key();
        msg!("user {user_public_key} favourate number is {number}, color is {color}");
        ctx.accounts.favourates.set_inner(Favourites {
            number, color, hobbies
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = ANCHOR_DESCRIMINATOR_SIZE+Favourites::INIT_SPACE, seeds = [b"favourates", user.key().as_ref()], bump)]
    pub favourates: Account<'info, Favourites>,
    pub system_program: Program<'info, System>

}