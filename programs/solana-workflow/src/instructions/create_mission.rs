use anchor_lang::prelude::*;

use crate::pda::*;

#[derive(Accounts)]
#[instruction(mission_id: u64, vote_data_id: u64)]
pub struct CreateMission<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed, 
        payer = user, 
        space=1000,
        seeds=[b"mission", user.key().as_ref(), &mission_id.to_le_bytes()], 
        bump
    )]
    pub mission: Account<'info, Mission>,
    #[account(
        init_if_needed, 
        payer = user, 
        space=1000,
        seeds=[b"vote_data", mission.key().as_ref(), &vote_data_id.to_le_bytes(), &[0]], 
        bump
    )]
    /// CHECK:
    pub vote_data: Account<'info, VoteData>,
    #[account()]
    /// CHECK:
    pub workflow_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_mission<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, CreateMission<'info>>,
    workflow_id: u64,
    mission_id: u64,
    title: String,
    content: String,
    current_vote_data: Pubkey,
    checkpoint_id: u16,
    vote_data_id: u64,
) -> Result<()> {
    let mission = &mut ctx.accounts.mission;
    Mission::create(
        mission,
        workflow_id,
        mission_id,
        title,
        content,
        current_vote_data,
    )?;
    
    let vote_data = &mut ctx.accounts.vote_data;
    // cpi to vote_machine
    vote_data.checkpoint_id = checkpoint_id;
    vote_data.id = vote_data_id;

    // for to create variable
    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter();
    let mut index = 0;
    for variable in remaining_accounts_iter {
        Variable::initialize(ctx.accounts.user.to_account_info(),
        variable,
        ctx.accounts.mission.to_account_info(),
        ctx.accounts.workflow_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        vec![0],
        index)?;

        index +=1;
    }
    Ok(())
}
