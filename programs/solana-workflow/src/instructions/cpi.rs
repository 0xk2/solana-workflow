use anchor_lang::{prelude::*, system_program::CreateAccount};

pub fn create_account<'info>(
    system_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    workflow: AccountInfo<'info>,
    id: u16,
    space: usize,
    owner: &Pubkey,
) -> Result<()> {
    msg!("Create_account to {:?}", to.key());
    anchor_lang::system_program::create_account(
        CpiContext::new(system_program, CreateAccount { from, to }).with_signer(&[&[
            &id.to_le_bytes(),
            b"checkpoint",
            workflow.key.as_ref(),
            &[253],
        ]]),
        Rent::get()?.minimum_balance(space),
        space.try_into().unwrap(),
        owner,
    )
}
