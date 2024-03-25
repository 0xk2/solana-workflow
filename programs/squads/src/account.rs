use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateTxn<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    /// CHECK:
    pub multisig: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK:
    pub transaction: AccountInfo<'info>,

    /// CHECK:
    pub squads_spl: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
