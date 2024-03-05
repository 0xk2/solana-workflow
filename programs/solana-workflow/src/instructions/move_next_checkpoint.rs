use anchor_lang::prelude::*;

use crate::pda::{CheckPoint, Mission, VoteData};

#[derive(Accounts)]
#[instruction(_vote_data_id: u16)]
pub struct MoveNextCheckpoint<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mission: Account<'info, Mission>,

    #[account()]
    pub next_checkpoint: Account<'info, CheckPoint>,

    #[account()]
    pub current_checkpoint: Account<'info, CheckPoint>,

    #[account(
        init_if_needed, 
        payer = user, 
        space=1000,
        seeds=[b"vote_data", mission.key().as_ref(), &_vote_data_id.to_le_bytes(), &[0]], 
        bump
    )]
    pub next_vote_data: Account<'info, VoteData>,

    #[account()]
    pub current_vote_data: Account<'info, VoteData>,

    pub system_program: Program<'info, System>,
}

pub fn move_next_checkpoint(ctx: Context<MoveNextCheckpoint>, _vote_data_id: u16) -> Result<()> {

    let mission = &mut ctx.accounts.mission;
    let next_vote_data = &mut ctx.accounts.next_vote_data;
    let next_checkpoint = &ctx.accounts.next_checkpoint;


    mission.current_vote_data = next_vote_data.key().clone();
    next_vote_data.checkpoint_id = next_checkpoint.id;

    Ok(())
}
