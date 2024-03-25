use anchor_lang::prelude::*;

use squads_spl::cpi::accounts::CreateTransaction;

use squads_spl::cpi::create_transaction;

use crate::account::CreateTxn;


pub fn create_txn<'c: 'info, 'info>(
    ctx: Context<'_, '_, 'c, 'info, CreateTxn<'info>>,
    authority_index: u32,
) -> Result<()> {
    let creact_txn_accounts = CreateTransaction {
        multisig: ctx.accounts.multisig.to_account_info(),
        transaction: ctx.accounts.transaction.to_account_info(),
        creator: ctx.accounts.user.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let create_txn_context = CpiContext::new(
        ctx.accounts.squads_spl.to_account_info(),
        creact_txn_accounts,
    );

    let _txn = create_transaction(create_txn_context, authority_index).unwrap();

    Ok(())
}
