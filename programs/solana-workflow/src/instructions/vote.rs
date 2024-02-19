use anchor_lang::prelude::*;

#[derive(Accounts)]
struct Vote {}

pub fn vote(ctx: Context<Vote>) -> Result<()> {
  Ok(())
}