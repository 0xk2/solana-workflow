use anchor_lang::prelude::*;

#[account]
pub struct TmpVoteData {
    pub id: u64,
    pub checkpoint_id: u16,
}