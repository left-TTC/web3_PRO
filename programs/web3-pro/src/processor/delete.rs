use crate::cpi::Cpi;
use crate::utils::Utils;
use crate::Web3_delete_Accounts;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


pub fn delete_process(ctx: Context<Web3_delete_Accounts>) -> ProgramResult{
    Utils::check_delete_key(&ctx)?;

    let reverse_key = Utils::get_reverse_key(&ctx)?;
    Utils::check_account_key(&ctx.accounts.reverse_lookup, &reverse_key)?;

    let seeds = ctx.accounts.name_account.key.to_bytes();
    //find auction state
    let (acution, _) = Pubkey::find_program_address(&[&seeds, &[1u8, 1u8]], ctx.program_id);
    Utils::check_account_key(&ctx.accounts.auction_state, &acution)?;

    //CPI delete reverse and domain
    Cpi::delete_account(&ctx)?;

    Ok(())
}