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

    pub fn create_domain(ctx: Context<Web3CreateAccounts>) -> ProgramResult {
        processor::create_domain(ctx)
    }
    
    pub fn delete_domain(ctx: Context<Web3DeleteAccounts>) -> ProgramResult {
        processor::delete_domain(ctx)
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
    root_domain_account: Option<Signer<'info>>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    name_account: UncheckedAccount<'info>,
    //solana program result
    system_program: Program<'info, System>,  
    //the vault 
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    vault: UncheckedAccount<'info>,  

    buyer: Signer<'info>,  
    //auction account
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    state: UncheckedAccount<'info>, 

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    reverse_lookup: UncheckedAccount<'info>,  

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    central_state: UncheckedAccount<'info>,  

    base_data: Account<'info, BaseInfo>,

    init_data: Account<'info, Web3Date>,
    //Providing token services
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    spl_token_program: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    rent_sysvar: UncheckedAccount<'info>,

    fee_payer: Signer<'info>,
    //user's token account
    buyer_token_source: Signer<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    referrer_opt: Option<UncheckedAccount<'info>>,
}


#[derive(Accounts)]
pub struct Web3DeleteAccounts<'info> {
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    web3_name_service: UncheckedAccount<'info>,  

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    system_program: Program<'info, System>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    root_domain_account: UncheckedAccount<'info>,  
    //domain account
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    name_account: UncheckedAccount<'info>,

    owner: Signer<'info>, 

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    reverse_lookup: UncheckedAccount<'info>, 

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    refund_targhet: UncheckedAccount<'info>,

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

#[account]
pub struct Web3Date{
    pub ipfs: Option<Vec<u8>>,
}

#[account]
pub struct BaseInfo{
    pub lamports: u64,
    pub space: u32,
    pub name: String,
}




