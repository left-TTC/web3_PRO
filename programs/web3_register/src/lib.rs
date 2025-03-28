use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::system_program;
use anchor_spl::token;
use constant::Constants;
use web3nameservice::program::Web3NameService;




declare_id!("7MReDm6FiS3n4A1sxTxdHu8p92TQutQSws715azLqtYj");




pub mod processor;
pub mod constant;
pub mod cpi;
pub mod utils;

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
    #[account(address = Constants::WEB_NAMEING_SERVICE)]
    web3_name_service: Program<'info, Web3NameService>,  
    
    // root domain(Y--common domain  N--root domain)
    /// CHECK: This account is verified in the instruction logic to ensure its safety
    #[account(owner = system_program::ID)]
    root_domain_account: Option<UncheckedAccount<'info>>,

    // cpi: so we choose UncheckedAccount
    /// CHECK: This account is verified in the instruction logic to ensure its safety
    #[account(mut)]
    name_account: UncheckedAccount<'info>,

    //solana program result
    /// CHECK: This account is verified in the instruction logic to ensure its safety
    system_program: Program<'info, System>, 

    //the vault 
    // #[account(address = hard_coded)]
    // vault: UncheckedAccount<'info>,  

    /// CHECK: This account is verified in the instruction logic to ensure its safety
    #[account(mut)]
    buyer: Signer<'info>,  

    //auction account
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    //state: UncheckedAccount<'info>, 
 

    // we will use this to record the domains which a accounts have
    /// CHECK: This account is verified in the instruction logic to ensure its safety
    #[account(mut)]
    domain_record: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = fee_payer,
        space = 8 + 1 + 8,
        seeds = [b"central_state"],
        bump,
    )]
    pub central_state: Account<'info, central_state>,


    //Providing token services
    // #[account(address = &token::ID)]
    // spl_token_program: UncheckedAccount<'info>,


    //rent_sysvar: UncheckedAccount<'info>,

    #[account(mut)]
    fee_payer: Signer<'info>,
    //user's token account

    //buyer_token_source: UncheckedAccount<'info>,


    //referrer_opt: Option<UncheckedAccount<'info>>,

}

#[account]
pub struct check_owner_account{
   //use to check on-chain account
   
}

#[account]
pub struct central_state{
    pub total_domain: u64,
    pub is_init: bool,
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








