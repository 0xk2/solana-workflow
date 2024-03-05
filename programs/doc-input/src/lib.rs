use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod states;
declare_id!("D1gMCgf8gHdUNDmpUfe1fHuUQci2JJFCw7CGv184hNMv");

#[program]
pub mod doc_input {
    use super::*;

    pub fn vote<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, Vote<'info>>,
        vote: InputVote,
    ) -> Result<()> {
        instructions::vote(ctx, vote)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
