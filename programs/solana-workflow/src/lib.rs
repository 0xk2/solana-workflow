use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod pda;

declare_id!("YiQingywsU6FotchaG7HpB9oYGhUn26gufxB8xrwF1s");

#[program]
pub mod solana_workflow {
    use super::*;

    pub fn create_workflow<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, CreateWorkflow<'info>>,
        title: String,
        start: u16,
        workflow_id: u64,
        input_checkpoints: Vec<InputCheckPoint>,
    ) -> Result<()> {
        instructions::create_workflow::create_workflow(
            ctx,
            title,
            start,
            workflow_id,
            input_checkpoints,
        )
    }
}

#[derive(Accounts)]
pub struct Initialize {}
