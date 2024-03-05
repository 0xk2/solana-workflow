use anchor_lang::prelude::*;

use solana_workflow::cpi::accounts::MoveNextCheckpoint;
use solana_workflow::pda::{CheckPoint, Mission, VoteData};

use solana_workflow::cpi:: move_next_checkpoint;

use crate::TmpVoteData;

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mission: Account<'info, Mission>,

    #[account()]
    pub checkpoint: Account<'info, CheckPoint>,

    /// CHECK:
    pub dash: AccountInfo<'info>,

    /// CHECK:
    pub vote_data: Account<'info, VoteData>,

    #[account(
        init_if_needed, 
        payer=user, 
        space=1000, 
        seeds=[b"tmp_vote_data", mission.key().as_ref(), vote_data.key().as_ref()], 
        bump
    )]
    pub tmp_vote_data: Account<'info, TmpVoteData>,

    pub system_program: Program<'info, System>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InputVote {
    pub option: u16,
    pub who: Pubkey
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Data {
    pub threshole: u8,
}

pub fn vote<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, Vote<'info>>,
    vote: InputVote,
) -> Result<()> {
    let current_checkpoint = &ctx.accounts.checkpoint;
    let tmp_vote_data = &mut ctx.accounts.tmp_vote_data;
    
    if tmp_vote_data.who.is_empty() {
        tmp_vote_data.who.push(vote.who);
        // Update result
        let result = &mut tmp_vote_data.result;
        match &current_checkpoint.options {
            Some(option) => {
                *result = vec![0; option.len()];
                result[vote.option as usize] = 1;
            },
            None => {}
        }
    } else {
        // check if user is already vote
        if tmp_vote_data.who.contains(&vote.who) {
            msg!("Already voted");
            return Ok(());
        } else {
            tmp_vote_data.who.push(vote.who);
            let result = &mut tmp_vote_data.result;
            result[vote.option as usize] += 1;
           
        }
    }

    
  
    let  checkpoint = &ctx.accounts.checkpoint;
    match &checkpoint.data {
        Some(data) => {    
            let data =  Data::try_from_slice(data).unwrap();
            
            if tmp_vote_data.result[vote.option as usize] == data.threshole {
                msg!("Tally");

                match &current_checkpoint.options {
                    Some(option) => {
                        // move to next checkpoint
                        let cpi_accounts_move = MoveNextCheckpoint {
                            user: ctx.accounts.user.to_account_info(),
                            mission: ctx.accounts.mission.to_account_info(),
                            next_checkpoint: ctx.remaining_accounts[vote.option as usize].to_account_info(),
                            current_checkpoint: ctx.accounts.checkpoint.to_account_info(),
                            next_vote_data: ctx.remaining_accounts[(vote.option * 2 + 1) as usize]
                                .to_account_info(),
                            current_vote_data: ctx.accounts.vote_data.to_account_info(),
                            system_program: ctx.accounts.system_program.to_account_info(),
                        };
            
                        let cpi_move = CpiContext::new(ctx.accounts.dash.to_account_info(), cpi_accounts_move);
                        let _cm = move_next_checkpoint(cpi_move, option[vote.option as usize].next_id).unwrap();
                    }
                    None => {
                        // Stop mission
                        msg!("Stop mission")
                    }
                }

            } else {
                msg!("Vote succesfully");
            }
        },
        None => {}
    }
    Ok(())
}
