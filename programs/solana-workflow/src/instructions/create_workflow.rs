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
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InputCheckPoint {
    id: u16,
    title: String,
    options: Option<Vec<VoteOption>>,
}

pub fn create_workflow(
    ctx: Context<CreateWorkflow>,
    title: String,
    start: u16,
    input_checkpoints: Vec<InputCheckPoint>,
) -> Result<()> {
    let workflow = &mut ctx.accounts.workflow;
    workflow.title = title.clone();
    workflow.start = start;
    workflow.author = ctx.accounts.user.key();

    for (index, account) in ctx.remaining_accounts.iter().enumerate() {
        let _account_key = account.key();
        let mut data = account.try_borrow_mut_data()?;

        let mut chkp =
            CheckPoint::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");

        chkp.title = input_checkpoints[index as usize].title.clone();
        chkp.id = input_checkpoints[index as usize].id.clone();
        chkp.options = input_checkpoints[index as usize].options.clone();

        chkp.try_serialize(&mut data.as_mut())?;
    }

    Ok(())
}
