use anchor_lang::{ prelude::*};


declare_id!("5UCAW1GRyMK6gcMwhZmaG3fWb5ocsXBBm6W9DdYuyiVL");


pub mod processor;
pub mod constant;
pub mod cpi;
pub mod utils;
pub mod error;

#[program]
pub mod web3_pro {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create_domain(ctx: Context<Web3_create_Accounts>) -> ProgramResult {
        processor::create_domain(ctx)
    }
    
    pub fn delete_domain(ctx: Context<Web3_delete_Accounts>) -> ProgramResult {
        processor::delete_domain(ctx)
    }


}


//‘info means it is effective in the lifetime of the programs
//all of these info should be accounts
#[derive(Accounts)]
pub struct Web3_create_Accounts<'info> {
    // web3 name service Program ID
    web3_name_service: UncheckedAccount<'info>,  
    // root domain(Y--common domain  N--root domain)
    root_domain_account: Option<Signer<'info>>,

    name_account: UncheckedAccount<'info>,
    //solana program result
    system_program: Program<'info, System>,  
    //the vault 
    vault: UncheckedAccount<'info>,  

    buyer: Signer<'info>,  
    //auction account
    state: UncheckedAccount<'info>, 

    reverse_lookup: UncheckedAccount<'info>,  

    central_state: UncheckedAccount<'info>,  

    base_data: Account<'info, base_info>,

    init_data: Account<'info, web3_data>,
    //Providing token services
    spl_token_program: UncheckedAccount<'info>,

    rent_sysvar: UncheckedAccount<'info>,

    fee_payer: Signer<'info>,
    //user's token account
    buyer_token_source: Signer<'info>,

    referrer_opt: Option<UncheckedAccount<'info>>,
}


#[derive(Accounts)]
pub struct Web3_delete_Accounts<'info> {
    web3_name_service: UncheckedAccount<'info>,  

    system_program: Program<'info, System>,

    root_domain_account: UncheckedAccount<'info>,  
    //domain account
    name_account: UncheckedAccount<'info>,

    owner: Signer<'info>, 

    reverse_lookup: UncheckedAccount<'info>, 

    refund_targhet: UncheckedAccount<'info>,

    //?
    //record the status of assets resealing
    /*such as 
        price
        shelf time
        has it been sold?
        profit attribution
     */
    resealing_state: UncheckedAccount<'info>,

    central_state: UncheckedAccount<'info>, 

    auction_state: UncheckedAccount<'info>,
}

#[account]
pub struct web3_data{
    pub ipfs: Option<Vec<u8>>,
}

#[account]
pub struct base_info{
    pub lamports: u64,
    pub space: u32,
    pub name: String,
}


