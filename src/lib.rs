#![feature(hash_raw_entry)]
#![feature(once_cell)]

pub mod instructions;
mod symbol_table;
use std::{lazy::SyncLazy, sync::RwLock};

pub use symbol_table::SymbolTable;
pub mod error_types {
    pub use super::symbol_table::SymbolTableError;
}

pub static SYMBOL_TABLE: SyncLazy<RwLock<SymbolTable>> = SyncLazy::new(RwLock::default);

/// A HackInt is an integer between 0 and 32767 (inclusive).
/// So technically a u16 is one bit larger but it is an in-built type we can use.
/// However, a HackInt shall always be inside of the aforementioned bounds.
/// We assure the correctness of this by checking the user input inside of the parser.
pub type HackInt = u16;
