use anchor_lang::{ prelude::*};

declare_id!("5UCAW1GRyMK6gcMwhZmaG3fWb5ocsXBBm6W9DdYuyiVL");

pub mod processor;
pub mod constant;

#[program]
pub mod web3_pro {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create_domain(ctx: Context<Web3Accounts>) -> ProgramResult {
        processor::create_domain(ctx)
    }

    pub fn query_domain(ctx: Context<Web3Accounts>) -> ProgramResult {
        processor::query_domain(ctx)
    }

    pub fn delete_domain(ctx: Context<Web3Accounts>) -> ProgramResult {
        processor::delete_domain(ctx)
    }

    pub fn transfer_domain(ctx: Context<Web3Accounts>)  -> ProgramResult {
        processor::transfer_domain(ctx)
    }
}


//‘info means it is effective in the lifetime of the programs
//all of these info should be accounts
#[derive(Accounts)]
pub struct Web3Accounts<'info> {
    // spl name service Program ID
    pub spl_name_service: UncheckedAccount<'info>,  

    // root domain
    pub root_domain_account: UncheckedAccount<'info>,  

    #[account(mut)]
    pub domain_name_account: Account<'info, DomainAccount>,  

    pub system_program: Program<'info, System>,  

    pub vault: UncheckedAccount<'info>,  

    pub buyer: Signer<'info>,  

    #[account(mut)]
    pub state: Account<'info, AuctionState>,  

    pub reverse_lookup: UncheckedAccount<'info>,  

    pub central_state: UncheckedAccount<'info>,  
}

#[account]
pub struct DomainAccount{
    pub owner: Pubkey,
    pub name: String,
}

#[account]
pub struct AuctionState{
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
}

