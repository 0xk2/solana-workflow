use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Workflow {
  pub author: Pubkey,
  pub workflow_id: u64,
  pub start: u16,
  #[max_len(50)]
  pub title: String,
}

impl Workflow {
  pub const SEED_PREFIX: &'static [u8; 8] = b"workflow";
  pub fn increase(&mut self) {
  }
}

#[account]
#[derive(InitSpace)]
pub struct VoteOption {
  #[max_len(10)]
  pub title: String,
  pub next_id: u16,
}

#[account]
#[derive(InitSpace)]
pub struct CheckPoint{
  pub workflow_id: u64,
  pub id: u16,
  #[max_len(50)]
  pub title: String,
  #[max_len(10)]
  pub options: Vec<VoteOption>,
}

impl CheckPoint {
  pub const SEED_PREFIX: &'static [u8; 10] = b"checkpoint";
  pub fn increase(&mut self) {
  }
}