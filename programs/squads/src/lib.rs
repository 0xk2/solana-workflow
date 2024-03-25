use anchor_lang::prelude::*;

pub mod account;
pub mod instructions;
pub mod state;

use account::*;

declare_id!("CoB5dX4re6TLvjopSi7WSV8wYPx2YD4FW5CP8B6YPF6W");

#[program]
pub mod squads {
    use super::*;

    pub fn create_txn<'c: 'info, 'info>(
        ctx: Context<'_, '_, 'c, 'info, CreateTxn<'info>>,
        authority_index: u32,
    ) -> Result<()> {
        instructions::create_txn(ctx, authority_index)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
