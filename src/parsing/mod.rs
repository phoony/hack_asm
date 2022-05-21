use crate::{
    hack_int::ParseHackIntError,
    instructions::{AInstruction, CInstruction},
};

mod a_instruction;
mod c_instruction;
mod label;
mod parser;

use parser::Rule;
use thiserror::Error;

pub enum ParsedInstruction<'a> {
    AInstruction(AInstruction<'a>),
    CInstruction(CInstruction),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    ParseHackIntError(#[from] ParseHackIntError),
    #[error(transparent)]
    PestError(#[from] pest::error::Error<Rule>),
}

pub use parser::parse_str;
