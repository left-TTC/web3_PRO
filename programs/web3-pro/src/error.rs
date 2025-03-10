use anchor_lang::*;
use anchor_lang::solana_program::decode_error::DecodeError;
use thiserror::Error;
use num_derive::FromPrimitive;


#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Error {
    #[error("Overflow")]
    Overflow,
}