//this file we will create funtion to interact with the name service
use anchor_lang::prelude::*;
use crate::Web3CreateAccounts;
use crate::Web3DeleteAccounts;
use anchor_lang::solana_program::{
    program::invoke_signed,
    instruction::Instruction,
};
use borsh::{BorshDeserialize, BorshSerialize};
use web3nameservice::cpi::accounts::create_name_service;
use web3nameservice::base_data;


pub mod Cpi{
    use anchor_lang::solana_program::{entrypoint::ProgramResult};

    use crate::storageData;

    use super::*;

    pub fn create_name_account(
        ctx: &Context<Web3CreateAccounts>,
        hashed_name: Vec<u8>,
        data: storageData,
    ) -> ProgramResult {
        //CPI call the web3 name service program

        msg!("name account: {}", ctx.accounts.name_account.key);
        msg!("payer: {}", ctx.accounts.buyer.key);

        //construct the cpi context
        let CPI_ctx = CpiContext::new(
            ctx.accounts.web3_name_service.to_account_info(),
            create_name_service {
                name_account: ctx.accounts.name_account.to_account_info(),
                system_account: ctx.accounts.system_program.to_account_info(),
                payer: ctx.accounts.buyer.to_account_info(),
                root_domain_opt: None,
            }
        );

        let CPI_data= base_data{
            lamports: 10000000,
            hashed_name: hashed_name,
            space: 0, 
            owner: data.owner,
            ipfs: data.ipfs,
        };

        msg!("start cpi");
        web3nameservice::cpi::create(CPI_ctx, CPI_data)?;
        
        Ok(())
    }


    pub fn deal_record_account(
        ctx: &Context<Web3CreateAccounts>,
        owner: Pubkey,
        domain: String,
        signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        

        Ok(())
    }

    pub fn delete_account (
        ctx: &Context<Web3DeleteAccounts>,
    ) -> ProgramResult {


        Ok(())
    }

}












