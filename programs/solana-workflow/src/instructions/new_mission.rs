use anchor_lang::prelude::*;

pub fn new_mission(ctx: Context<NewMission>) -> Result<()> {
  Ok(())
}

#[derive(Accounts)]
struct NewMission {}