#![feature(hash_raw_entry)]

mod instructions;
mod symbol_table;

/// A HackInt is an integer between 0 and 32767 (inclusive).
/// So technically a u16 is one bit larger but it is an in-built type we can use.
/// However, a HackInt shall always be inside of the aforementioned bounds.
/// We assure the correctness of this by checking the user input inside of the parser.
type HackInt = u16;
