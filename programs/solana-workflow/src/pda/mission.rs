use anchor_lang::prelude::*;

#[account]
struct Mission{
    workflow_id: u64,
    id: u64,
    title: String,
    content: String,
}

#[account]
struct VoteData{
    checkpoint_id: u64,
}
