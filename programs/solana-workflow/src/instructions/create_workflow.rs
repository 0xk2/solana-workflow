use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;
use anchor_lang::solana_program::slot_history::Check;
use crate::pda;
use crate::workflow;
use pda::workflow::Workflow;
use pda::workflow::CheckPoint;
use pda::workflow::VoteOption;

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
  pub system_program: Program<'info, System>,
}

#[account]
pub struct InputCheckPoint{
  id: u16,
  title: String,
  options: Vec<String>,
}
#[account]
pub struct InputVoteOption{
  pub title: String,
  pub next_id: u16,
}

pub fn create_workflow(ctx: Context<CreateWorkflow>, title:String, start: u16, inputCheckpoints: Vec<InputCheckPoint>) -> Result<()> {
  let checkpointAccounts: &&[CheckPoint<'_>] = &ctx.remaining_accounts;
  let workflow = &mut ctx.accounts.workflow;
  workflow.title = title.clone();
  workflow.start = start;
  workflow.author = ctx.accounts.user.key();
  let mut seed_data = Vec::new();
  for inputChkp in inputCheckpoints.iter() {
    let chkp: &mut CheckPoint = &mut checkpointAccounts[inputChkp.id as usize];
    chkp.title = inputChkp.title.clone();
    for inputOpt in inputChkp.options.iter() {
      let opt = &mut chkp.options.push(VoteOption{
        title: inputOpt.clone(),
        next_id: 0,
      });
      seed_data.extend_from_slice(&opt.title.as_bytes());
    }
    chkp.options = inputChkp.options.iter().map(|opt| VoteOption{
      title: opt.clone(),
      next_id: 0,
    }).collect();
  } 
  Ok(())
}