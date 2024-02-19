use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Vote {}

pub fn vote(_ctx: Context<Vote>) -> Result<()> {
  Ok(())
} 