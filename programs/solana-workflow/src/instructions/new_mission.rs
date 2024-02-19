use anchor_lang::prelude::*;

pub fn new_mission(_ctx: Context<NewMission>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct NewMission {}
