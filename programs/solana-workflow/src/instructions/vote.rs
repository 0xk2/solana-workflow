use anchor_lang::prelude::*;

use crate::pda::{CheckPoint, Mission, Status, VoteData};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mission: Account<'info, Mission>,

    #[account()]
    pub checkpoint: Account<'info, CheckPoint>,

    /// CHECK:
    pub workflow_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InputVote {
    pub option: u16,
}

pub fn vote<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, Vote<'info>>,
    vote: InputVote,
    vec_coef: Vec<u8>,
) -> Result<()> {
    let mission = &mut ctx.accounts.mission;
    let checkpoint = &ctx.accounts.checkpoint;

    // let next_check_point_id = checkpoint.options[vote.option as usize];

    match &checkpoint.options {
        Some(options) => {
            let next_checkpoint_id = options[vote.option as usize].next_id;

            mission.current_vote_data = ctx.remaining_accounts[vote.option as usize].key();

            VoteData::initialize(
                ctx.accounts.user.to_account_info(),
                &ctx.remaining_accounts[vote.option as usize],
                mission.to_account_info(),
                ctx.accounts.workflow_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                vec_coef[vote.option as usize].into(),
                next_checkpoint_id,
                vec_coef[vote.option as usize],
            )?;
        }
        None => {
            mission.status = Status::CLOSED;
        }
    }

    Ok(())
}
