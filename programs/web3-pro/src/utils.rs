//This is a method library
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::Web3CreateAccounts;
use crate::constant::Constants;
use crate::Web3DeleteAccounts;
use anchor_lang::solana_program::system_program;
use anchor_spl::token;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::hash::hashv;
use unicode_segmentation::UnicodeSegmentation;



pub mod Utils{

    use super::*;
    
    //check account keys
    pub fn check_account_key(account: &AccountInfo, key: &Pubkey) -> ProgramResult {
        if account.key != key {
            #[cfg(feature = "Debug")]
            msg!("Wrong account key: {} should be {}", account.key, key);
            return Err(ProgramError::InvalidArgument);
        } 
        Ok(())
    }

    //check signature: 
    //Verify whether the specified account signs the transaction
    fn check_signer (account: &AccountInfo) -> ProgramResult {
        if !(account.is_signer) {
            #[cfg(feature = "Debug")]
            msg!("Missing signature for: {}", account.key);
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }

    pub fn check_account_owner(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
        if account.owner != owner {
            msg!("Wrong account owner: {} should be {}", account.owner, owner);
            return Err(ProgramError::InvalidArgument);
        }
        Ok(())
    }

    //external interface
    pub fn create_create_check(ctx: &Context<Web3CreateAccounts>) -> ProgramResult{
        //Check the incoming domain name service contract
        check_account_key (&ctx.accounts.web3_name_service, &Constants::WEB_NAMEING_SERVICE)?;
        //
        if let Some(root) = &ctx.accounts.root_domain_account{
            check_account_key(root, &Constants::ROOT_DOMAIN_ACCOUNT)?;
        }
        //
        check_account_key(&ctx.accounts.system_program, &system_program::ID)?;
        //check central_state account
        //check_account_key(&ctx.accounts.central_state, &central_state::key)?;
        //
        check_account_key(&ctx.accounts.spl_token_program, &token::ID)?;
        //
        check_account_key(&ctx.accounts.rent_sysvar, &sysvar::ID)?;
        
        //The owner of the name account must be a system program
        //the domain owner will be recorded in account's data
        check_account_owner(&ctx.accounts.name_account, &system_program::ID)?;
        //the vault account should be owned by the SPL token 
        check_account_owner(&ctx.accounts.vault, &token::ID)?;
        //state is auction account
        check_account_owner(&ctx.accounts.state, &system_program::ID)?;

        check_signer( &ctx.accounts.buyer)?;
        check_signer(&ctx.accounts.fee_payer)?;

        Ok(())
    }

    pub fn check_delete_key (ctx: &Context<Web3DeleteAccounts>) -> ProgramResult {
        check_account_key(&ctx.accounts.web3_name_service, &Constants::WEB_NAMEING_SERVICE)?;
        check_account_key(&ctx.accounts.system_program, &system_program::ID)?;
        //central_state


        //check the account owner: web3 naming service or current program
        check_account_owner(&ctx.accounts.name_account, &Constants::WEB_NAMEING_SERVICE)
            .or_else(|_| check_account_owner(&ctx.accounts.name_account, &ctx.program_id))?;
        check_account_owner(&ctx.accounts.reverse_lookup, &Constants::WEB_NAMEING_SERVICE)
            .or_else(|_| check_account_owner(&ctx.accounts.reverse_lookup, &ctx.program_id))?;
        //check the resealing account
        check_account_owner(&ctx.accounts.resealing_state, &system_program::ID)
            .or_else(|_| check_account_owner(&ctx.accounts.resealing_state, &*ctx.program_id))?;
        check_account_owner(&ctx.accounts.auction_state, &system_program::ID)
            .or_else(|_| check_account_owner(&ctx.accounts.auction_state, &*ctx.program_id))?;

        check_signer(&ctx.accounts.owner)?;

        Ok(())
    }






    //calculate name's hased value 
    pub fn get_hashed_name(name: &str) -> Vec<u8> {
        hashv(&[(Constants::HASH_PREFIX.to_owned() + name).as_bytes()])
            .as_ref()
            .to_vec()
    }

    //program_id: the id of web3 name service
    //hashed_name: the hased value of domain
    //if use root, means it's common domain
    pub fn get_seeds_and_key(
        program_id: &Pubkey,
        hashed_name: Vec<u8>,
        root_opt: &Option<Pubkey>,
    ) -> (Pubkey, Vec<u8>) {
        //hashed name as the init seeds
        let mut seeds_vec: Vec<u8> = hashed_name;
        //root domain(when create a root domian,use default)
        let root_domian = root_opt.clone().unwrap_or_default();
        //add root to the sed
        for b in root_domian.to_bytes() {
            seeds_vec.push(b);
        }
    
        let (name_account_key, bump) =
            Pubkey::find_program_address(&seeds_vec.chunks(32).collect::<Vec<&[u8]>>(), program_id);
        seeds_vec.push(bump);
    
        (name_account_key, seeds_vec)
    }

    //calculate the domain's PDA
    pub fn get_name_key (ctx: &Context<Web3CreateAccounts>) -> Result<Pubkey> {
        let hashed_name = get_hashed_name(&ctx.accounts.base_data.name);
        let root_key = if let Some(root) = &ctx.accounts.root_domain_account {
            Some(*root.key)
        }else{
            None
        };
        let (name_account_key, _) = get_seeds_and_key(
            &Constants::WEB_NAMEING_SERVICE,
            hashed_name,
            &root_key,
        );
        Ok(name_account_key)
    }

    //calculate the reverse account's PDA
    pub fn get_reverse_key (ctx: &Context<Web3DeleteAccounts>) -> Result<Pubkey> {
        //seeds composition: domain account's pubkey
        let hased_reverse_look_up = get_hashed_name(&ctx.accounts.name_account.key.to_string());
        let (reverse_lookup_key, _) = get_seeds_and_key(
            ctx.program_id,
            hased_reverse_look_up,
            &Some(*ctx.accounts.root_domain_account.key));

        Ok(reverse_lookup_key)
    }






    //Correctly counting the number of "graphemes" in a string
    fn get_grapheme_len(name: &str) -> usize {
        name.graphemes(true).count()
    }

    fn get_usd_price (len: usize) -> u64 {
        let multiplier = match len {
            1 => 750,
            2 => 700,
            3 => 640,
            4 => 160,
            _ => 20,
        };
        #[cfg(not(feature = "devnet"))]
        return multiplier * 1_000_000;
        #[cfg(feature = "devnet")]
        return multiplier * 1_000;
    }

    pub fn get_domian_price_checked (ctx: &Context<Web3CreateAccounts>) -> Result<u64>{
        let usd_price = get_usd_price(get_grapheme_len(&ctx.accounts.base_data.name));
        //get buyer's token type
        //this is a kind of account that created by 
        /*pub struct Account {
            pub mint: Pubkey,                    //the token type in this account     
            pub owner: Pubkey,     
            pub amount: u64,         
            pub state: AccountState, 
            pub is_native: Option<u64>, 
            pub delegated_amount: u64,  
            pub close_authority: Option<Pubkey>, 
        } */
       //get the mint
        let buyer_token_mint = 
            get_spl_Account_mint(&ctx.accounts.buyer_token_source.data.borrow())?;
        
        //connet pyth and get the value

        let test_value: u64 = 7;
        Ok(test_value)
    }

    pub fn get_spl_Account_mint (spl_account_data: &[u8]) -> Result<Pubkey>{
        if spl_account_data.len() < 32 {
            return Err(ProgramError::InvalidAccountData.into());
        }

        let mint_bytes: [u8; 32] = spl_account_data[..32]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(Pubkey::new_from_array(mint_bytes))
    }

    pub fn get_spl_Account_owner (spl_account_data: &[u8]) -> Result<Pubkey>{
        if spl_account_data.len() < 64 {
            return Err(ProgramError::InvalidAccountData.into());
        }

        let owner_bytes: [u8; 32] = spl_account_data[32..64]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(Pubkey::new_from_array(owner_bytes))
    }




    //check designated referrer's discount
    pub fn get_special_discount_and_fee (referrer_key: &Pubkey) -> (Option<u8>, Option<u8>) {
        (Some(1), Some(2))
    }









    
}

