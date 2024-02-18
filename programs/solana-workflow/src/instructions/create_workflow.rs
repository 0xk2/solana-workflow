use anchor_lang::prelude::*;
use pda::Workflow;
use pda::CheckPoint;

#[derive(Accounts)]
pub struct CreateWorkflow {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    init_if_needed,
    payer = user,
    space = Workflow::SPACE,
    seeds = [Workflow::SEED_PREFIX, user.key().as_ref()],
    bump,
  )]
  pub workflow: Account<'info, Workflow>,
  pub system_program: Program<'info, System>,
}

pub fn create_workflow(ctx: Context<CreateWorkflow>) -> Result<()> {
  // each checkpoint is a CheckPoint
  let checkpoints = &ctx.remaining_accounts;
  Ok(())
}