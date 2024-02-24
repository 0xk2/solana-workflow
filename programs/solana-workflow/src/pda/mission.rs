use anchor_lang::prelude::*;

#[account]
pub struct Mission {
    pub workflow_id: u64,
    pub id: u64,
    pub title: String,
    pub content: String,
    pub current_vote_data: Pubkey,
}
impl Mission {
    pub fn create(
        &mut self,
        workflow_id: u64,
        id: u64,
        title: String,
        content: String,
        current_vote_data: Pubkey,
    ) -> Result<()> {
        self.workflow_id = workflow_id;
        self.id = id;
        self.title = title;
        self.content = content;
        self.current_vote_data = current_vote_data;
        Ok(())
    }
}

#[account]
pub struct VoteData {
    pub id: u64,
    pub checkpoint_id: u16,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Variable {}
