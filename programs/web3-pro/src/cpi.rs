//this file we will create funtion to interact with the name service
use anchor_lang::prelude::*;
use crate::Web3_create_Accounts;
use crate::Web3_delete_Accounts;


pub mod Cpi{
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create_name_account(
        ctx: &Context<Web3_create_Accounts>,
        hashed_name: Vec<u8>,
        //signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        //CPI call the web3 name service program

        Ok(())
    }

    pub fn create_reverse_lookup_account(
        ctx: &Context<Web3_create_Accounts>,
        //signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        //CPI call the web3 name service program --- create

        Ok(())
    }

    pub fn delete_account (
        ctx: &Context<Web3_delete_Accounts>,
    ) -> ProgramResult {


        Ok(())
    }

}












