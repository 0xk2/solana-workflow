use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Workflow {
  pub author: Pubkey,
  pub workflow_id: u64,
}

impl Workflow {
  pub const SEED_PREFIX: &'static [u8; 8] = b"workflow";
  pub fn increase(&mut self) {
  }
}

#[account]
pub struct VoteOption {
  pub title: String,
  pub next_id: u64,
}

#[account]
struct CheckPoint{
  pub workflow_id: u64,
  pub id: u64,
  pub options: Vec<VoteOption>,
}

impl CheckPoint {
  pub const SEED_PREFIX: &'static [u8; 10] = b"checkpoint";
  pub fn increase(&mut self) {
  }
}