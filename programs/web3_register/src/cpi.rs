//this file we will create funtion to interact with the name service
use anchor_lang::prelude::*;
use crate::Web3CreateAccounts;
use crate::Web3DeleteAccounts;


pub mod Cpi{
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create_name_account(
        ctx: &Context<Web3CreateAccounts>,
        hashed_name: Vec<u8>,
        //signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        //CPI call the web3 name service program

        Ok(())
    }

    pub fn create_reverse_lookup_account(
        ctx: &Context<Web3CreateAccounts>,
        //signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        //CPI call the web3 name service program --- create

        Ok(())
    }

    pub fn delete_account (
        ctx: &Context<Web3DeleteAccounts>,
    ) -> ProgramResult {


        Ok(())
    }

}












