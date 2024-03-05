use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod pda;

declare_id!("GfV9B4DHq93LCWjD3nSmFgStH7BxrDka8v7fgH7u1SCV");

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

    pub fn create_mission(
        ctx: Context<CreateMission>,
        workflow_id: u64,
        mission_id: u64,
        title: String,
        content: String,
        current_vote_data: Pubkey,
        checkpoint_id: u16,
        vote_data_id: u64,
    ) -> Result<()> {
        instructions::create_mission::create_mission(
            ctx,
            workflow_id,
            mission_id,
            title,
            content,
            current_vote_data,
            checkpoint_id,
            vote_data_id,
        )
    }

    pub fn move_next_checkpoint(ctx: Context<MoveNextCheckpoint>, vote_data_id: u16) -> Result<()> {
        instructions::move_next_checkpoint::move_next_checkpoint(ctx, vote_data_id)
    }

    pub fn change_variable(
        ctx: Context<CreateVariable>,
        value: Vec<u8>,
        variable_id: u8,
    ) -> Result<()> {
        instructions::change_variable::change_variable(ctx, value, variable_id)
    }
}
