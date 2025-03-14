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
        name: String,
        ipfs: Option<Vec<u8>>,
        ) -> ProgramResult {
        processor::create_domain(ctx, name, ipfs)
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
    root_domain_account: Option<Signer<'info>>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
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
    reverse_lookup: UncheckedAccount<'info>,  

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



//unit test
/*****************        TEST         ********************/
#[cfg(test)]
mod test {
    use crate::constant::Constants::{self, WEB_NAMEING_SERVICE};
    use crate::utils::Utils;

    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::pubkey::Pubkey;
    use anchor_lang::solana_program::{lamports, system_program};
    use anchor_spl::token;
    use anchor_lang::solana_program::epoch_schedule::Epoch;
    use anchor_lang::solana_program::sysvar;

    #[test]
    fn test_statement() {
        msg!("this is the unit test in the lib.rs");
        msg!("I'd like to test all my function here");
        let id = crate::ID;
        msg!("now programID: {}", id);
    }

    #[test]
    fn test_create_domain() {
        msg!("start create test");

        let name = "web3".to_string();

        //confirmed: this is program id
        let program_id = crate::ID;
        let buyer = Pubkey::new_unique();
        let fee_payer = Pubkey::new_unique();
        
        let buyer_token_source = Pubkey::new_unique();
        //Request the public key using a specific method
        let namekey = Utils::get_name_key(None, &name).unwrap();
        let name_account = namekey;
        //temporary
        let reverse_lookup_id = Utils::get_reverse_key(
            &program_id, &name_account, central_state::KEY).unwrap();

        let  data: Vec<u8> = Vec::new();
        let  lamports = 100;

        //Impersonate the data of the token_source account
        let usdc_mint = pubkey!("3RjZG5PbE55Y9wzvqXt9GiFFK5oU4WkZ7FPToaHb5zjh");
        let owner = Pubkey::new_from_array([2u8; 32]); 
        let amount: u64 = 1000; 

        let mut data_mut1 = data.clone();
        let mut data_mut2 = data.clone();
        let mut data_mut3 = data.clone();
        let mut data_mut4 = data.clone();
        let mut data_mut5 = data.clone();
        let mut data_mut6 = data.clone();
        let mut data_mut7 = data.clone();
        let mut data_mut8 = data.clone();
        let mut data_mut9 = data.clone();
        data_mut9.extend_from_slice(&usdc_mint.to_bytes());
        data_mut9.extend_from_slice(&owner.to_bytes());
        data_mut9.extend_from_slice(&amount.to_be_bytes());
        let mut data_mut10 = data.clone();
        let mut data_mut11 = data.clone();

        let mut lamports_mut1 = lamports.clone();
        let mut lamports_mut2 = lamports.clone();
        let mut lamports_mut3 = lamports.clone();
        let mut lamports_mut4 = lamports.clone();
        let mut lamports_mut5 = lamports.clone();
        let mut lamports_mut6 = lamports.clone();
        let mut lamports_mut7 = lamports.clone();
        let mut lamports_mut8 = lamports.clone();
        let mut lamports_mut9 = lamports.clone();
        let mut lamports_mut10 = lamports.clone();
        let mut lamports_mut11 = lamports.clone();


        let buyer_accountinfo = generate_account(
            &buyer, true, false, &mut lamports_mut1,  & mut data_mut1, &system_program::ID, false);

        let fee_payerAccountInfo = generate_account(
            &fee_payer, true, false, &mut lamports_mut2,  & mut data_mut2, &system_program::ID, false);
        
        let web3_name_service_account = generate_account(
            &Constants::WEB_NAMEING_SERVICE, false, false, &mut lamports_mut3, &mut data_mut3, &system_program::ID, false
        );
        
        let name_account_info = generate_account(
            &name_account, false, false, &mut lamports_mut4, &mut data_mut4, &WEB_NAMEING_SERVICE, false
        );
        
        let vault_account_info = generate_account(
            &Constants::VAULT_ACCOUNT, false, false, &mut lamports_mut5, &mut data_mut5, &token::ID, false
        );
        
        let state_account_info = generate_account(
            &Constants::AUCTION_PROGRAM_ID, false, false, &mut lamports_mut6, &mut data_mut6, &system_program::ID, false
        );
        
        let spl_token_program_account = generate_account(
            &token::ID, false, false, &mut lamports_mut7, &mut data_mut7, &system_program::ID, false
        );
        
        let rent_sysvar_account = generate_account(
            &sysvar::ID, false, false, &mut lamports_mut8, &mut data_mut8, &system_program::ID, false
        );
        
        let buyer_token_source_account = generate_account(
            &buyer_token_source, false, false, &mut lamports_mut9, &mut data_mut9, &system_program::ID, false
        ); 

        let central_state = generate_account(
            &central_state::KEY, false, false, &mut lamports_mut10, &mut data_mut10, &program_id, false
        );

        let reverse_lookup = generate_account(
            &reverse_lookup_id, false, false, &mut lamports_mut11, &mut data_mut11, &Constants::WEB_NAMEING_SERVICE, false
        );
        
        let mut accounts = Web3CreateAccounts {
            web3_name_service: UncheckedAccount::try_from(&web3_name_service_account),
            root_domain_account: None,
            name_account: UncheckedAccount::try_from(&name_account_info),
            vault: UncheckedAccount::try_from(&vault_account_info),
            buyer: Signer::try_from(&buyer_accountinfo).unwrap(),
            state: UncheckedAccount::try_from(&state_account_info),
            spl_token_program: UncheckedAccount::try_from(&spl_token_program_account),
            rent_sysvar: UncheckedAccount::try_from(&rent_sysvar_account),
            fee_payer: Signer::try_from(&fee_payerAccountInfo).unwrap(),
            referrer_opt: None,
            buyer_token_source: UncheckedAccount::try_from(&buyer_token_source_account),
            central_state: UncheckedAccount::try_from(&central_state),
            reverse_lookup: UncheckedAccount::try_from(&reverse_lookup),
        };
        
        let ctx = Context::new(&program_id, &mut accounts, &[], Default::default());

        msg!("start test");
        create_domain(ctx, name, None);


    }

    fn generate_account<'a>(
        pubkey: &'a Pubkey, 
        signer: bool,
        write: bool,
        lamports: &'a mut u64,
        data: &'a mut [u8],
        owner: &'a Pubkey,
        executable: bool,
    ) -> AccountInfo<'a> {
        AccountInfo::new(
            pubkey,
            signer,
            write,
            lamports,
            data,
            owner,
            executable,
            Epoch::default(),
        )
    }

    #[test]
    pub fn test_delete_domain() {
        msg!("start delete domain");


        let  data: Vec<u8> = Vec::new();
        let  lamports = 100;

        let mut data_mut1 = data.clone();
        let mut data_mut2 = data.clone();
        let mut data_mut3 = data.clone();
        let mut data_mut4 = data.clone();
        let mut data_mut5 = data.clone();
        let mut data_mut6 = data.clone();
        let mut data_mut7 = data.clone();
        let mut data_mut9 = data.clone();

        let mut lamports_mut1 = lamports.clone();
        let mut lamports_mut2 = lamports.clone();
        let mut lamports_mut3 = lamports.clone();
        let mut lamports_mut4 = lamports.clone();
        let mut lamports_mut5 = lamports.clone();
        let mut lamports_mut6 = lamports.clone();
        let mut lamports_mut7 = lamports.clone();
        let mut lamports_mut9 = lamports.clone();

        //confirmed: this is program id
        let program_id = crate::ID;
        let root_domain = Pubkey::new_unique();

        //Request the public key using a specific method
        let name = "f".to_string();
        let namekey = Utils::get_name_key(Some(root_domain), &name).unwrap();
        let name_account = namekey;
        let refund_key = Pubkey::new_unique();

        let delete_owner = Pubkey::new_unique();

        let seeds = name_account.to_bytes();
        let (state, _) = Pubkey::find_program_address(&[&seeds], &program_id);

        let (reselling_state, _) = Pubkey::find_program_address(&[&seeds, &[1u8, 1u8]], &program_id);

        //temporary
        let reverse_lookup_id = Utils::get_reverse_key(
            &program_id, &name_account, central_state::KEY).unwrap();

        let web3_name_service_account = generate_account(
            &Constants::WEB_NAMEING_SERVICE, false, false, &mut lamports_mut3, &mut data_mut3, &system_program::ID, false
        );
        
        let name_account_info = generate_account(
            &name_account, false, false, &mut lamports_mut4, &mut data_mut4, &Constants::WEB_NAMEING_SERVICE, false
        );
          
        let state_account_info = generate_account(
            &state, false, false, &mut lamports_mut6, &mut data_mut6, &system_program::ID, false
        );
        
        let root_domain_account = generate_account(
            &root_domain, false, false, &mut lamports_mut9, &mut data_mut9, &system_program::ID, false
        ); 

        let central_state = generate_account(
            &central_state::KEY, false, false, &mut lamports_mut1, &mut data_mut1, &program_id, false
        );

        let reverse_lookup = generate_account(
            &reverse_lookup_id, false, false, &mut lamports_mut2, &mut data_mut2, &Constants::WEB_NAMEING_SERVICE, false
        );

        let owner_info = generate_account(
            &delete_owner, true, false, &mut lamports_mut5, &mut data_mut5, &system_program::ID, false
        );

        let reseal_account = generate_account(
            &reselling_state, false, false, &mut lamports_mut7, &mut data_mut7, &system_program::ID, false
        );

        let mut delet_account = Web3DeleteAccounts{
            web3_name_service: UncheckedAccount::try_from(&web3_name_service_account),
            root_domain_account: UncheckedAccount::try_from(&root_domain_account),
            name_account: UncheckedAccount::try_from(&name_account_info),
            central_state: UncheckedAccount::try_from(&central_state),
            reverse_lookup: UncheckedAccount::try_from(&reverse_lookup),
            auction_state: UncheckedAccount::try_from(&state_account_info),
            owner: Signer::try_from(&owner_info).unwrap(),
            resealing_state :UncheckedAccount::try_from(&reseal_account),
        };

        msg!("create delete test accounts over");
        let ctx = Context::new(&program_id, &mut delet_account, &[], Default::default());

        delete_domain(ctx,refund_key);

    }

    
}






