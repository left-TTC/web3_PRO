use crate::{central_state, constant::Constants};
use crate::Web3CreateAccounts;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::utils::Utils;
use anchor_spl::token;
use crate::error::Error;
use anchor_spl::token::{Transfer, transfer};
use crate::cpi::Cpi;


pub fn create_process(
    ctx: Context<Web3CreateAccounts>,
    name: String,
    ipfs: Option<Vec<u8>>
) -> ProgramResult{
    Utils::create_check(&ctx)?;
    //make sure is's only lowecase
    #[cfg(feature = "Debug")]
    msg!("now we check domain/root: {}",name);
    
    if name != 
        name.trim().to_lowercase() {
        #[cfg(feature = "Debug")]
        msg!("Domain names must be lower case and have no space");
        return Err(ProgramError::InvalidArgument);
    }

    //without "."
    if name.contains(".") {
        #[cfg(feature = "Debug")]
        msg!("format err");
        return Err(ProgramError::InvalidArgument);
    }

    //get the root domain key
    let root_domain_account_key = if let Some(value) = &ctx.accounts.root_domain_account {
        Some(*value.key)
    }else{
        None
    };
    let name_account_key = Utils::get_name_key(root_domain_account_key, &name)?;
    
    if &name_account_key != ctx.accounts.name_account.key {
        #[cfg(feature = "Debug")]   
        msg!("Provided wrong name account");
        return Err(ProgramError::InvalidArgument);
    }


    //find auction account on solana
    let (auction_state_key, _) = 
        Pubkey::find_program_address(&[&name_account_key.to_bytes()], ctx.program_id);
    if &auction_state_key != ctx.accounts.state.key {
        #[cfg(feature = "Debug")]
        msg!("An invalid name auctioning state account was provided");
        msg!("we don't test it now");
        //return Err(ProgramError::InvalidArgument);
    }

    if !ctx.accounts.state.data_is_empty() {
        #[cfg(feature = "Debug")]
        msg!("The name auctioning state account is not empty.");
        return Err(ProgramError::InvalidArgument);
    }

    //compare reverse lookup key
    let reverse_lookup_key = Utils::get_reverse_key(
        ctx.program_id, ctx.accounts.name_account.key, central_state::KEY)?;
    if &reverse_lookup_key != ctx.accounts.reverse_lookup.key {
        msg!("Provided wrong reverse lookup account");
        return Err(ProgramError::InvalidArgument);
    }

    //we will use it at cpi process
    let central_state_signer_seeds: &[&[u8]] = &[&ctx.program_id.to_bytes(), &[central_state::NONCE]];

    //calculate the price and return the token type
    let (mut domain_token_price, token_acc) = Utils::get_domian_price_checked(
        &ctx.accounts.buyer_token_source, &name)?;
    msg!("domain_price: {}", domain_token_price);
    
    //referrer policy
    #[cfg(feature = "devnet")]
    let referrer_fee = 
        if let Some(referrer_account) = &ctx.accounts.referrer_opt {
            //referrer's account should be owned by spl token;
            Utils::check_account_owner(&referrer_account.to_account_info(), &token::ID)?;
            //get the recommended fee ratio
            let mut referrer_fee_pct = Constants::REFERRER_FEE_PCT;
            //parse the Referrer Token Account and get the owner
            let referrer_token_owner_key = Utils::get_spl_Account_owner(&referrer_account.data.borrow())?;
            //check whitelist

            //get referral discounts and special fee percentages
            let (discount, special_fee) = 
                Utils::get_special_discount_and_fee(&referrer_token_owner_key);
            //apply discount
            if let Some(discount) = discount {
                domain_token_price = 100u64
                    .checked_sub(discount as u64)
                    .ok_or(Error::Overflow)?
                    .checked_mul(domain_token_price)
                    .ok_or(Error::Overflow)?
                    / 100;
            };
            if let Some(special_fee) = special_fee {
                referrer_fee_pct = special_fee as u64
            }

            let referrer_fees_amount = domain_token_price.checked_mul(referrer_fee_pct).unwrap() / 100;

            //transfer the referr fee to referrer
            //Construct a Transfer structure
            let transfer_info = Transfer{
                from: ctx.accounts.buyer_token_source.to_account_info(),
                to: referrer_account.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            };
            let cpi_program = ctx.accounts.spl_token_program.to_account_info();
            //package as ctx
            let transfer_ctx = CpiContext::new(cpi_program, transfer_info);
            //package as ctx and execute transfer
            let transfer_ix = transfer(
                transfer_ctx,
                referrer_fees_amount,
            )?;
            //return the amount
            referrer_fees_amount
    }else{
        0
    };

    //transfer amount to vault
    #[cfg(feature = "devnet")]
    transfer_to_vault(ctx)?;

    msg!("transfer OK");

    //create domain name -- CPI call -> web3 naming service
    //fristly: get the rent info form solana
    let rent = Rent::get();
    let hashed_name = Utils::get_hashed_name(&name);

    msg!("send create instruction to web3 name service");
    Cpi::create_name_account(
        &ctx, hashed_name, central_state_signer_seeds )?;

    //cpi call --> create reverse_look_up account
    msg!("send create_reverse instruction to web3 name service");
    Cpi::create_reverse_lookup_account(&ctx)?;

    msg!("create domain over");
    Ok(())
}

fn transfer_to_vault (ctx: Context<Web3CreateAccounts>, domain_token_price: u64) -> ProgramResult {
    let vault_transfer_info = Transfer {
        from: ctx.accounts.buyer_token_source.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };

    let cpi_program = ctx.accounts.spl_token_program.to_account_info();

    let vault_transfer_ctx = CpiContext::new(cpi_program, vault_transfer_info);

    transfer(
        vault_transfer_ctx,
        domain_token_price,
    )?;

    Ok(())
}

