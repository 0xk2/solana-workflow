use anchor_lang::prelude::*;
use instructions::*;
use pda::*;

pub mod instructions;
pub mod pda;

declare_id!("XkVX49m9A2fLQ92jW6KZStypy5wsrAeZ5PndJeUn31c");

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
