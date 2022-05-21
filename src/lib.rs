#![feature(hash_raw_entry)]
#[macro_use]
extern crate pest_derive;

mod assembler;
mod assembler_context;
mod hack_int;
mod instructions;
mod parsing;
mod symbol_table;

pub use assembler::Assembler;

mod constants {
    use crate::hack_int::HackInt;

    pub(crate) const MEMORY_SIZE: HackInt = HackInt::new_unchecked(16383);
    pub(crate) const ROM_SIZE: usize = 32767;
}
