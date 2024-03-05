use anchor_lang::prelude::*;
use states::*;
use instructions::*;

pub mod instructions;
pub mod states;

declare_id!("3XR9BbkbddGNFCbEG59XXEy9MydHZmGZb6jEq4VxQWY7");

#[program]
pub mod single_choice {
    use super::*;

    pub fn vote<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, Vote<'info>>,
        vote: InputVote,
        vec_coef: Vec<u8>,
    ) -> Result<()> {
        instructions::vote::vote(ctx, vote, vec_coef)
    }
}
