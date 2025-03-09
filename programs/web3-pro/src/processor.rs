use anchor_lang::prelude::*;
use crate::{ Web3_create_Accounts, Web3_delete_Accounts};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use create::create_process;
use delete::delete_process;

pub mod create;
pub mod delete;

pub fn create_domain(ctx: Context<Web3_create_Accounts>) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("create instruction");
    create_process(ctx)
}


pub fn delete_domain(ctx: Context<Web3_delete_Accounts>) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("delete instruction");
    delete_process(ctx)
}
