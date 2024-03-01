use anchor_lang::prelude::*;

#[account]
pub struct Mission {
    pub workflow_id: u64,
    pub id: u64,
    pub title: String,
    pub content: String,
    pub current_vote_data: Pubkey,
    pub status: Status,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Status {
    CLOSED,
    STARTED,
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
        self.status = Status::STARTED;
        Ok(())
    }
}
#[account]
pub struct Variable {
    pub value: Vec<u8>,
}
