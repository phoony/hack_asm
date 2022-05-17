#![feature(hash_raw_entry)]

mod hack_int;
mod instructions;
mod symbol_table;

mod constants {
    pub(crate) const MEMORY_SIZE: usize = 16383;
    pub(crate) const ROM_SIZE: usize = 32767;
}
