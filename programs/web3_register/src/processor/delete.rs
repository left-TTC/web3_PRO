use crate::cpi::Cpi;
use crate::utils::Utils;
use crate::{central_state, Web3DeleteAccounts};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


pub fn delete_process(
    ctx: Context<Web3DeleteAccounts>,
    refund_target: Pubkey
    ) -> ProgramResult{
    Utils::check_delete_key(&ctx)?;
    
    //check reverse_account
    let reverse_key = Utils::get_reverse_key(
        ctx.program_id, ctx.accounts.name_account.key, central_state::KEY)?;
    Utils::check_account_key(
        ctx.accounts.reverse_lookup.key, &reverse_key)?;
    msg!("reverse account ok");

    //check auction state
    let seeds = ctx.accounts.name_account.key.to_bytes();
    let (acution, _) = Pubkey::find_program_address(&[&seeds], ctx.program_id);
    Utils::check_account_key(ctx.accounts.auction_state.key, &acution)?;
    msg!("auction state ok");

    //check reseal state
    let (reselling_state, _) = Pubkey::find_program_address(&[&seeds, &[1u8, 1u8]], ctx.program_id);
    Utils::check_account_key(ctx.accounts.resealing_state.key, &reselling_state)?;
    msg!("reselling state ok");

    //CPI delete reverse and domain
    Cpi::delete_account(&ctx)?;

    msg!("delete over");
    Ok(())
}