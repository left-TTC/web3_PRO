use crate::{constant::Constants};
use crate::{storageData, Web3CreateAccounts};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::utils::Utils;
use anchor_spl::token;
use anchor_spl::token::{Transfer, transfer};
use crate::cpi::Cpi;


pub fn create_process(
    ctx: Context<Web3CreateAccounts>,
    create_data: storageData,
) -> ProgramResult{
    //Utils::create_check(&ctx)?;
    msg!("now we check domain/root: {}",create_data.name);
    let mut a = "none";
    if !ctx.accounts.central_state.is_init{
        a = "false"
    }else if ctx.accounts.central_state.is_init {
        a = "true"
    }
    msg!("central_state: {}", a);
    
    if create_data.name != 
        create_data.name.trim().to_lowercase() {
        #[cfg(feature = "Debug")]
        msg!("Domain names must be lower case and have no space");
        return Err(ProgramError::InvalidArgument);
    }

    //without "."
    if create_data.name.contains(".") {
        #[cfg(feature = "Debug")]
        msg!("format err");
        return Err(ProgramError::InvalidArgument);
    }
    msg!("name format is ok");

    let mut if_root = false;

    //get the root domain key
    let root_domain_key = if let Some(value) = &ctx.accounts.root_domain_account {
        msg!("root: {}", value.key);
        msg!("payer: {}", ctx.accounts.buyer.key);
        if value.key == ctx.accounts.buyer.key
            || value.key == &Pubkey::default() {
            msg!("be the payer");
            if_root = true;
            None
        }else{
            if_root = true;
            Some(value.key)
        }
    }else{
        msg!("root is none, ok");
        None
    };

    //manuallt check
    let name_account_key = Utils::get_PDA_key(
        root_domain_key, &create_data.name,)?;
    
    if &name_account_key != ctx.accounts.name_account.key {
        #[cfg(feature = "Debug")]   
        msg!("Provided wrong name account");
        return Err(ProgramError::InvalidArgument);
    }

    //compare reverse lookup key
    //root domain does't need record
    let owner_record_key = Utils::get_PDA_key(
        root_domain_key,
        &create_data.owner.to_string(),
        )?;
    if &owner_record_key != ctx.accounts.domain_record.key {
        msg!("Provided wrong owner record account");
        return Err(ProgramError::InvalidArgument);
    }


    // find auction account on solana
    // let (auction_state_key, _) = 
    //     Pubkey::find_program_address(&[&name_account_key.to_bytes()], ctx.program_id);
    // if &auction_state_key != ctx.accounts.state.key {
    //     #[cfg(feature = "Debug")]
    //     msg!("An invalid name auctioning state account was provided");
    //     msg!("we don't test it now");
    //     //return Err(ProgramError::InvalidArgument);
    // }

    // if !ctx.accounts.state.data_is_empty() {
    //     #[cfg(feature = "Debug")]
    //     msg!("The name auctioning state account is not empty.");
    //     return Err(ProgramError::InvalidArgument);
    // }

    
    //we will use it at cpi process
    let central_bump = ctx.bumps.central_state;
    let central_state_signer_seeds: &[&[u8]] = &[
        b"central_state",     
        &[central_bump], 
    ];

    //calculate the price and return the token type
    // let (mut domain_token_price, token_acc) = Utils::get_domian_price_checked(
    //     &ctx.accounts.buyer_token_source, &create_data.name)?;
    // msg!("domain_price: {}", domain_token_price);
    
    //referrer policy
    // #[cfg(feature = "devnet")]
    // let referrer_fee = 
    //     if let Some(referrer_account) = &ctx.accounts.referrer_opt {
    //         //referrer's account should be owned by spl token;
    //         Utils::check_account_owner(&referrer_account.to_account_info(), &token::ID)?;
    //         //get the recommended fee ratio
    //         let mut referrer_fee_pct = Constants::REFERRER_FEE_PCT;
    //         //parse the Referrer Token Account and get the owner
    //         let referrer_token_owner_key = Utils::get_spl_Account_owner(&referrer_account.data.borrow())?;
    //         //check whitelist

    //         //get referral discounts and special fee percentages
    //         let (discount, special_fee) = 
    //             Utils::get_special_discount_and_fee(&referrer_token_owner_key);
    //         //apply discount
    //         if let Some(discount) = discount {
    //             domain_token_price = 100u64
    //                 .checked_sub(discount as u64)
    //                 .ok_or(Error::Overflow)?
    //                 .checked_mul(domain_token_price)
    //                 .ok_or(Error::Overflow)?
    //                 / 100;
    //         };
    //         if let Some(special_fee) = special_fee {
    //             referrer_fee_pct = special_fee as u64
    //         }

    //         let referrer_fees_amount = domain_token_price.checked_mul(referrer_fee_pct).unwrap() / 100;

    //         //transfer the referr fee to referrer
    //         //Construct a Transfer structure
    //         let transfer_info = Transfer{
    //             from: ctx.accounts.buyer_token_source.to_account_info(),
    //             to: referrer_account.to_account_info(),
    //             authority: ctx.accounts.buyer.to_account_info(),
    //         };
    //         let cpi_program = ctx.accounts.spl_token_program.to_account_info();
    //         //package as ctx
    //         let transfer_ctx = CpiContext::new(cpi_program, transfer_info);
    //         //package as ctx and execute transfer
    //         let transfer_ix = transfer(
    //             transfer_ctx,
    //             referrer_fees_amount,
    //         )?;
    //         //return the amount
    //         referrer_fees_amount
    // }else{
    //     0
    // };

    //transfer amount to vault
    // #[cfg(feature = "devnet")]
    // transfer_to_vault(&ctx, domain_token_price)?;

    msg!("transfer OK");

    //create domain name -- CPI call -> web3 naming service
    //fristly: get the rent info form solana
    let rent = Rent::get();
    let hashed_name = Utils::get_hashed_name(&create_data.name);



    msg!("send create instruction to web3 name service");
    Cpi::create_name_account(
        &ctx, 
        hashed_name, 
        central_state_signer_seeds, 
        create_data.clone())?;

    //cpi call --> create reverse_look_up account
    // msg!("send domain_record instruction to web3 name service");
    // if ctx.accounts.domain_record.data.borrow().len() == 0 {
    //     msg!("this is a new user");
    //     Cpi::deal_record_account(
    //         &ctx, 
    //         create_data.owner, 
    //         create_data.name.clone(), 
    //         central_state_signer_seeds)?;
    // }

    msg!("create domain over");

    let central_state_account = &mut ctx.accounts.central_state;

    if !central_state_account.is_init {
        central_state_account.is_init = true;
        central_state_account.total_domain = 0;
    }else {
        central_state_account.total_domain += 1;
    }

    Ok(())
}

// fn transfer_to_vault (ctx: &Context<Web3CreateAccounts>, domain_token_price: u64) -> ProgramResult {
//     let vault_transfer_info = Transfer {
//         from: ctx.accounts.buyer_token_source.to_account_info(),
//         to: ctx.accounts.vault.to_account_info(),
//         authority: ctx.accounts.buyer.to_account_info(),
//     };

//     let cpi_program = ctx.accounts.spl_token_program.to_account_info();

//     let vault_transfer_ctx = CpiContext::new(cpi_program, vault_transfer_info);

//     transfer(
//         vault_transfer_ctx,
//         domain_token_price,
//     )?;

//     Ok(())
// }

