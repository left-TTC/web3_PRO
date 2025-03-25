use anchor_lang::{ prelude::*};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use left_utils::declare_central_state;


declare_id!("Dbpxr1SxxmjpLBRAxwPqQ8JBZeRUU2LCyrThpLdnBTRY");
declare_central_state!("Dbpxr1SxxmjpLBRAxwPqQ8JBZeRUU2LCyrThpLdnBTRY");

pub mod processor;
pub mod constant;
pub mod cpi;
pub mod utils;
pub mod error;

#[program]
pub mod web3Regitser {
   

    use super::*;

    pub fn create_domain(
        ctx: Context<Web3CreateAccounts>,
        data: storageData,
        ) -> ProgramResult {
        processor::create_domain(ctx, data)
    }
    
    pub fn delete_domain(
        ctx: Context<Web3DeleteAccounts>,
        target: Pubkey
        ) -> ProgramResult {
        processor::delete_domain(ctx, target)
    }


}


//‘info means it is effective in the lifetime of the programs
//all of these info should be accounts
#[derive(Accounts)]
pub struct Web3CreateAccounts<'info> {
    // web3 name service Program ID
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    web3_name_service: UncheckedAccount<'info>,  
    
    // root domain(Y--common domain  N--root domain)
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    root_domain_account: Option<UncheckedAccount<'info>>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(mut)]
    name_account: UncheckedAccount<'info>,
    //solana program result
    //system_program: Program<'info, System>,  
    //the vault 
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    vault: UncheckedAccount<'info>,  

    buyer: Signer<'info>,  

    //auction account
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    state: UncheckedAccount<'info>, 

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    // it's Unnecessary
    //reverse_lookup: UncheckedAccount<'info>,  

    // we will use this to record the domains which a accounts have
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(mut)]
    domain_record: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    central_state: UncheckedAccount<'info>,  

    //Providing token services
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    spl_token_program: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    rent_sysvar: UncheckedAccount<'info>,

    fee_payer: Signer<'info>,
    //user's token account
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    buyer_token_source: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    referrer_opt: Option<UncheckedAccount<'info>>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    class: Signer<'info>,
}

#[account]
pub struct storageData{
    name: String,
    owner: Pubkey,
    ipfs: Option<Vec<u8>>
}



#[derive(Accounts)]
pub struct Web3DeleteAccounts<'info> {
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    web3_name_service: UncheckedAccount<'info>,  

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    //system_program: Program<'info, System>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    root_domain_account: UncheckedAccount<'info>,  
    //domain account
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    name_account: UncheckedAccount<'info>,

    owner: Signer<'info>, 

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    reverse_lookup: UncheckedAccount<'info>, 

    //?
    //record the status of assets resealing
    /*such as 
        price
        shelf time
        has it been sold?
        profit attribution
     */
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    resealing_state: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    central_state: UncheckedAccount<'info>, 

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    auction_state: UncheckedAccount<'info>,
}








