use anchor_lang::*;
use anchor_lang::solana_program::program_error::ProgramError;

use thiserror::Error;
use num_derive::FromPrimitive;


#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Error {
    #[error("Overflow")]
    Overflow,
}

impl From<crate::error::Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}



