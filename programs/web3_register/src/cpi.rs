//this file we will create funtion to interact with the name service
use anchor_lang::prelude::*;
use crate::Web3CreateAccounts;
use crate::Web3DeleteAccounts;
use anchor_lang::solana_program::{
    program::invoke_signed,
    instruction::Instruction,
};

use web3nameservice::{
    base_data,
};


pub mod Cpi{
    use anchor_lang::solana_program::{entrypoint::ProgramResult};

    use crate::storageData;

    use super::*;

    pub fn create_name_account(
        ctx: &Context<Web3CreateAccounts>,
        hashed_name: Vec<u8>,
        signer_seeds: &[&[u8]],
        data: storageData,
    ) -> ProgramResult {
        //CPI call the web3 name service program

        let create_domain_discriminator: [u8; 8] = [103, 208, 151, 155, 64, 18, 133, 109];

        let cpi_data = base_data{
            lamports: 100000,
            hashed_name: hashed_name,
            space: 200,
            owner: data.owner,
            ipfs: data.ipfs
        };

        let mut serialized_data = create_domain_discriminator.to_vec(); 
        serialized_data.extend_from_slice(&cpi_data.try_to_vec().unwrap());

        let mut accounts = vec![
            AccountMeta::new(
                *ctx.accounts.name_account.key, false),
            AccountMeta::new(
                *ctx.accounts.fee_payer.key, true),
            AccountMeta::new_readonly(
                *ctx.accounts.class.key, true),    
        ];

        if let Some(root_domain) = &ctx.accounts.root_domain_account{
            accounts.push((AccountMeta::new_readonly(*root_domain.key, false)));
        }else {
            accounts.push(AccountMeta::new_readonly(Pubkey::default(), false));
        }

        let cpi_instruction = Instruction {
            program_id: *ctx.accounts.web3_name_service.key,
            accounts: accounts,
            data: serialized_data,
        };

        let signer_seeds = [signer_seeds];

        invoke_signed(
            &cpi_instruction,
            &[
                ctx.accounts.web3_name_service.to_account_info(),
                ctx.accounts.name_account.to_account_info(),
                ctx.accounts.fee_payer.to_account_info(),
                ctx.accounts.class.to_account_info(),
                ctx.accounts.root_domain_account
                    .as_ref()
                    .map(|a| a.to_account_info())
                    .unwrap(),
            ],
            &signer_seeds
        )?;

        Ok(())
    }


    pub fn deal_record_account(
        ctx: &Context<Web3CreateAccounts>,
        owner: Pubkey,
        domain: String,
        signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        let create_domain_discriminator: [u8; 8] = [103, 208, 151, 155, 64, 18, 133, 109];

        //CPI call the web3 name service program --- create or update
        //frist get the domain bytes that will be added
        let mut storage_domain = Vec::new();
        let add_domain = domain.as_bytes();
        storage_domain.extend_from_slice(add_domain);

        //The update process will be executed directly

        let mut byte_owner: Vec<u8> = Vec::new();
        let owner_bytes = &owner.clone().to_bytes();
        byte_owner.extend_from_slice(owner_bytes);

        let cpi_data = base_data{
            lamports: 100000,
            hashed_name: byte_owner,
            space: 100,
            owner: owner,
            //there is the owner's domain array
            ipfs: Some(storage_domain)
        };

        let mut serialized_data = create_domain_discriminator.to_vec(); 
        serialized_data.extend_from_slice(&cpi_data.try_to_vec().unwrap());

        let mut accounts = vec![
            AccountMeta::new(
                *ctx.accounts.name_account.key, false),
            AccountMeta::new(
                *ctx.accounts.fee_payer.key, true),
            AccountMeta::new_readonly(
                *ctx.accounts.class.key, true),    
        ];

        if let Some(root_domain) = &ctx.accounts.root_domain_account{
            accounts.push((AccountMeta::new_readonly(*root_domain.key, false)));
        }else {
            accounts.push(AccountMeta::new_readonly(Pubkey::default(), false));
        }

        let cpi_instruction = Instruction {
            program_id: *ctx.accounts.web3_name_service.key,
            accounts: accounts,
            data: serialized_data,
        };

        let signer_seeds = [signer_seeds];

        invoke_signed(
            &cpi_instruction,
            &[
                ctx.accounts.web3_name_service.to_account_info(),
                ctx.accounts.name_account.to_account_info(),
                ctx.accounts.fee_payer.to_account_info(),
                ctx.accounts.class.to_account_info(),
                ctx.accounts.root_domain_account
                    .as_ref()
                    .map(|a| a.to_account_info())
                    .unwrap(),
            ],
            &signer_seeds
        )?;

        Ok(())
    }

    pub fn delete_account (
        ctx: &Context<Web3DeleteAccounts>,
    ) -> ProgramResult {


        Ok(())
    }

}












