use crate::pda;
use anchor_lang::prelude::*;
use pda::workflow::CheckPoint;
use pda::workflow::VoteOption;
use pda::workflow::Workflow;

#[derive(Accounts)]
pub struct CreateWorkflow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init_if_needed,
    payer = user,
    space = 8 + Workflow::INIT_SPACE,
    seeds = [Workflow::SEED_PREFIX, user.key().as_ref()],
    bump,
  )]
    pub workflow: Account<'info, Workflow>,
    #[account()]
    /// CHECK:
    pub workflow_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InputCheckPoint {
    id: u16,
    title: String,
    options: Option<Vec<VoteOption>>,
}

pub fn create_workflow<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, CreateWorkflow<'info>>,
    title: String,
    start: u16,
    workflow_id: u64,
    input_checkpoints: Vec<InputCheckPoint>,
) -> Result<()> {
    let _ = input_checkpoints;
    let workflow = &mut ctx.accounts.workflow;
    workflow.title = title.clone();
    workflow.start = start;
    workflow.workflow_id = workflow_id;
    workflow.author = ctx.accounts.user.key();

    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter();

    CheckPoint::initialize(
        ctx.accounts.user.to_account_info(),
        next_account_info(remaining_accounts_iter)?,
        ctx.accounts.workflow.to_account_info(),
        ctx.accounts.workflow_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        workflow_id,
        input_checkpoints[0].id,
        input_checkpoints[0].title.clone(),
        input_checkpoints[0].options.clone(),
    )?;

    msg!("Done");

    Ok(())
}
