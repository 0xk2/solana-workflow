use anchor_lang::{prelude::*, system_program::CreateAccount};

pub fn create_account<'info>(
    system_program: AccountInfo<'info>,
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
    space: usize,
    owner: &Pubkey,
) -> Result<()> {
    let seeds_signer = &mut seeds.to_vec();
    let binding = [bump];
    seeds_signer.push(&binding);

    // signer seeds must equal seeds of to address
    anchor_lang::system_program::create_account(
        CpiContext::new(system_program, CreateAccount { from, to }).with_signer(&[seeds_signer]),
        Rent::get()?.minimum_balance(space),
        space.try_into().unwrap(),
        owner,
    )
}
