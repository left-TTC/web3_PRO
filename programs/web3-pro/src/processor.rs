use anchor_lang::prelude::*;
use crate::Web3Accounts;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use create::create_process;
use delete::delete_process;
use query::query_process;
use transfer::transfer_process;

pub mod create;
pub mod query;
pub mod delete;
pub mod transfer;

pub fn create_domain(ctx: Context<Web3Accounts>) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("create instruction");
    create_process(ctx)
}

pub fn query_domain(ctx: Context<Web3Accounts>) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("query instruction");
    delete_process(ctx)
}

pub fn delete_domain(ctx: Context<Web3Accounts>) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("delete instruction");
    transfer_process(ctx)
}

pub fn transfer_domain(ctx: Context<Web3Accounts>) -> ProgramResult {
    #[cfg(feature = "Debug")]
    msg!("transfer instruction");
    query_process(ctx)
}