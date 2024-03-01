use anchor_lang::prelude::*;

use crate::{funcs, BpfWriter};

#[account]
pub struct VoteData {
    pub id: u64,
    pub checkpoint_id: u16,
}

impl VoteData {
    pub const SIZE: usize = 1000;

    fn from<'info>(x: &'info AccountInfo<'info>) -> Account<'info, Self> {
        Account::try_from_unchecked(x).unwrap()
    }

    pub fn serialize(&self, info: AccountInfo) -> Result<()> {
        let dst: &mut [u8] = &mut info.try_borrow_mut_data().unwrap();
        let mut writer: BpfWriter<&mut [u8]> = BpfWriter::new(dst);
        VoteData::try_serialize(self, &mut writer)
    }

    pub fn create(&mut self, id: u64, checkpoint_id: u16) -> Result<()> {
        self.id = id;
        self.checkpoint_id = checkpoint_id;
        Ok(())
    }

    pub fn initialize<'info>(
        payer: AccountInfo<'info>,
        vote_data: &'info AccountInfo<'info>,
        mission: AccountInfo<'info>,
        workflow_program: AccountInfo<'info>,
        system_program: AccountInfo<'info>,
        id: u64,
        checkpoint_id: u16,
        coef: u8,
    ) -> Result<()> {
        let seeds: &[&[u8]] = &[
            b"vote_data",
            mission.key.as_ref(),
            &checkpoint_id.to_le_bytes(),
            &[coef],
        ];

        let (_, bump) = Pubkey::find_program_address(seeds, &workflow_program.key());

        funcs::create_account(
            system_program,
            payer.to_account_info(),
            vote_data.to_account_info(),
            &seeds,
            bump,
            VoteData::SIZE,
            &workflow_program.key(),
        )?;

        // deserialize and modify checkpoint account
        let mut run = VoteData::from(&vote_data);
        run.create(id, checkpoint_id)?;

        // write
        run.serialize(vote_data.to_account_info())?;
        Ok(())
    }
}
