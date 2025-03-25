use anchor_lang::prelude::*;
use crate::{ storageData, Web3CreateAccounts, Web3DeleteAccounts};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use create::create_process;
use delete::delete_process;

pub mod create;
pub mod delete;

pub fn create_domain(
    ctx: Context<Web3CreateAccounts>,
    data: storageData,) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("create instruction");
    create_process(ctx, data)
}


pub fn delete_domain(
    ctx: Context<Web3DeleteAccounts>,
    refund_target: Pubkey
    ) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("delete instruction");
    delete_process(ctx, refund_target)
}

