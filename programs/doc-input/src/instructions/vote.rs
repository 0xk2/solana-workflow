use std::any::Any;

use anchor_lang::prelude::*;

use solana_workflow::cpi::accounts::{CreateVariable, MoveNextCheckpoint};
use solana_workflow::pda::{CheckPoint, Mission, VoteData};

use solana_workflow::cpi::{change_variable, move_next_checkpoint};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mission: Account<'info, Mission>,

    #[account()]
    pub checkpoint: Account<'info, CheckPoint>,

    /// CHECK:
    pub dash: AccountInfo<'info>,

    /// CHECK:
    pub vote_data: Account<'info, VoteData>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InputVote {
    pub option: u16,
    pub submission: Vec<u8>,
}

pub fn vote<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, Vote<'info>>,
    vote: InputVote,
    vec_coef: Vec<u8>,
) -> Result<()> {
    // save value to variable
    let cpi_change_variable = CreateVariable {
        user: ctx.accounts.user.to_account_info(),
        mission: ctx.accounts.mission.to_account_info(),
        variable: ctx.remaining_accounts[(vote.option * 2 + 2) as usize].to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let cpi_change_variable = CpiContext::new(
        ctx.accounts.dash.to_account_info(),
        cpi_change_variable,
    );

    let _ccv = change_variable(cpi_change_variable, vote.submission, 1).unwrap();

    // move to next checkpoint
    let cpi_accounts_move = MoveNextCheckpoint {
        user: ctx.accounts.user.to_account_info(),
        mission: ctx.accounts.mission.to_account_info(),
        next_checkpoint: ctx.remaining_accounts[vote.option as usize].to_account_info(),
        current_checkpoint: ctx.accounts.checkpoint.to_account_info(),
        next_vote_data: ctx.remaining_accounts[(vote.option * 2 + 1) as usize].to_account_info(),
        current_vote_data: ctx.accounts.vote_data.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let cpi_move = CpiContext::new(ctx.accounts.dash.to_account_info(), cpi_accounts_move);
    let _cm = move_next_checkpoint(cpi_move, 3).unwrap();

    Ok(())
}
