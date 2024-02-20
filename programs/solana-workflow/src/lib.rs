use anchor_lang::prelude::*;
use instructions::*;
use pda::*;

pub mod instructions;
pub mod pda;

declare_id!("4mZ5qV2oXUqhtzw6LeLeB3R24aUhq3bzXsqezkk2dUJj");

#[program]
pub mod solana_workflow {
    use super::*;

    pub fn create_workflow(
        ctx: Context<CreateWorkflow>,
        title: String,
        start: u16,
        input_checkpoints: Vec<InputCheckPoint>
    ) -> Result<()> {
        instructions::create_workflow::create_workflow(ctx, title, start, input_checkpoints)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
