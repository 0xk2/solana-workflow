use anchor_lang::prelude::*;

use crate::{funcs, BpfWriter};

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

impl Variable {
    pub const SIZE: usize = 1000;

    fn from<'info>(x: &'info AccountInfo<'info>) -> Account<'info, Self> {
        Account::try_from_unchecked(x).unwrap()
    }

    pub fn serialize(&self, info: AccountInfo) -> Result<()> {
        let dst: &mut [u8] = &mut info.try_borrow_mut_data().unwrap();
        let mut writer: BpfWriter<&mut [u8]> = BpfWriter::new(dst);
        Variable::try_serialize(self, &mut writer)
    }

    pub fn create(&mut self, value: Vec<u8>) -> Result<()> {
        self.value = value;
        Ok(())
    }

    pub fn initialize<'info>(
        payer: AccountInfo<'info>,
        variable: &'info AccountInfo<'info>,
        mission: AccountInfo<'info>,
        workflow_program: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
        value: Vec<u8>,
        index: u8,
    ) -> Result<()> {
        let seeds: &[&[u8]] = &[b"variable", mission.key.as_ref(), &[index]];

        let (_, bump) = Pubkey::find_program_address(seeds, &workflow_program.key());

        funcs::create_account(
            system_program,
            payer.to_account_info(),
            variable.to_account_info(),
            &seeds,
            bump,
            Variable::SIZE,
            &workflow_program.key(),
        )?;

        // deserialize and modify account
        let mut run = Variable::from(&variable);
        run.create(value)?;

        // write
        run.serialize(variable.to_account_info())?;
        Ok(())
    }
}
