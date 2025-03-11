use crate::constant::Constants;
use crate::Web3CreateAccounts;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::utils::Utils;
use anchor_spl::token;
use crate::error::Error;
use anchor_spl::token::{Transfer, transfer};
use crate::cpi::Cpi;


pub fn create_process(ctx: Context<Web3CreateAccounts>) -> ProgramResult{
    Utils::create_create_check(&ctx)?;
    //make sure is's only lowecase
    if ctx.accounts.base_data.name != 
        ctx.accounts.base_data.name.trim().to_lowercase() {
        #[cfg(feature = "Debug")]
        msg!("Domain names must be lower case and have no space");
        return Err(ProgramError::InvalidArgument);
    }
    //without "."
    if ctx.accounts.base_data.name.contains(".") {
        #[cfg(feature = "Debug")]
        msg!("format err");
        return Err(ProgramError::InvalidArgument);
    }
    
    let name_account_key = Utils::get_name_key(&ctx)?;
    
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
        return Err(ProgramError::InvalidArgument);
    }

    if !ctx.accounts.state.data_is_empty() {
        #[cfg(feature = "Debug")]
        msg!("The name auctioning state account is not empty.");
        return Err(ProgramError::InvalidArgument);
    }

    let hashed_reverse_lookup = Utils::get_hashed_name(&name_account_key.to_string());

    //get reverse lookup account's PDA with central state key
    //what is central state key?

    //compare reverse lookup key

    //calculate the price
    let mut domain_token_price = Utils::get_domian_price_checked(&ctx)?;
    //get mint 
    let token_acc = 
        Utils::get_spl_Account_mint(&ctx.accounts.buyer_token_source.data.borrow())?;
    //discount? if user use designated mint
    
    //referrer policy
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
    let vault_transfer_info = Transfer {
        from: ctx.accounts.buyer_token_source.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };

    let cpi_program = ctx.accounts.spl_token_program.to_account_info();

    let vault_transfer_ctx = CpiContext::new(cpi_program, vault_transfer_info);

    let valut_transfer_ix = transfer(
        vault_transfer_ctx,
        domain_token_price,
    );

    //create domain name -- CPI call -> web3 naming service
    //fristly: get the rent info form solana
    let rent = Rent::get();
    let hashed_name = Utils::get_hashed_name(&ctx.accounts.base_data.name);
    Cpi::create_name_account(&ctx, hashed_name/*, signer_seeds */)?;

    //cpi call --> create reverse_look_up account

    Cpi::create_reverse_lookup_account(&ctx)?;


    Ok(())
}

