use anchor_lang::prelude::*;

use crate::pda::{ Mission, Variable};

#[derive(Accounts)]
#[instruction(_variable_id: u8)]
pub struct CreateVariable<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mission: Account<'info, Mission>,

    #[account(mut)]
    pub variable: Account<'info, Variable>,

    pub system_program: Program<'info, System>,
}

pub fn change_variable(ctx: Context<CreateVariable>, value: Vec<u8>, _variable_id: u8) -> Result<()> {
    let variable = &mut ctx.accounts.variable;

    variable.value = value;

    Ok(())
}
