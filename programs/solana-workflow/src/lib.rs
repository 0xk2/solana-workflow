use anchor_lang::prelude::*;
use pda::*;
use instructions::*;

pub mod pda;
pub mod instructions;

declare_id!("GfV9B4DHq93LCWjD3nSmFgStH7BxrDka8v7fgH7u1SCV");

#[program]
pub mod solana_workflow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}