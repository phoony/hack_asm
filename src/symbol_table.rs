use std::collections::HashMap;

use crate::HackInt;

use phf::phf_map;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymbolTableError {
    #[error("Tried to redefine the built in symbol \"{0}\"")]
    RedefinedBuiltIn(String),
    #[error("Tried to redefine the symbol \"{0}\"")]
    Redefined(String),
    #[error("Symbol \"{0}\" not defined")]
    NotDefined(String),
}

static BUILT_IN: phf::Map<&'static str, HackInt> = phf_map! {
    // Virtual Registers
    "R0" =>  0,
    "R1" =>  1,
    "R2" =>  2,
    "R3" =>  3,
    "R4" =>  4,
    "R5" =>  5,
    "R6" =>  6,
    "R7" =>  7,
    "R8" =>  8,
    "R9" =>  9,
    "R10" => 10,
    "R11" => 11,
    "R12" => 12,
    "R13" => 13,
    "R14" => 14,
    "R15" => 15,
    // IO
    "SCREEN" => 16384,
    "KBD"    => 24576,
    // Reserved
    "SP"   => 0,
    "LCL"  => 1,
    "ARG"  => 2,
    "THIS" => 3,
    "THAT" => 4,
};

/// The symbol table stores and resolves symbols (labels and variables)
/// to their associated addresses or values.
///
/// # List of predefined symbols
///
/// Virtual Registers
/// ------------------------
/// | Symbol    | Value    |
/// |-----------|----------|
/// |    R0     |    0     |
/// |    R1     |    1     |
/// |    ...    |   ...    |
/// |    R15    |    15    |
/// ------------------------
///
/// Input/Output
/// ------------------------
/// | Symbol    | Value    |
/// | --------- | -------- |
/// | SCREEN    |  16384   |
/// | KBD       |  24576   |
/// ------------------------
///
/// Reserved
/// ------------------------
/// | Symbol    | Value    |
/// | --------- | -------- |
/// |    SP     |    0     |
/// |    LCL    |    1     |
/// |    ARG    |    2     |
/// |    THIS   |    3     |
/// |    That   |    4     |
/// ------------------------
/// # Example
/// ## Basic Usage
/// ```ignore
/// # use hack_asm::SymbolTable;
/// let mut table = SymbolTable::new();
/// table.set("value", 42).unwrap();
/// assert!(table.set("value", 101).is_err());      // a symbols' value may only be set once
///
/// assert_eq!(table.get("value").unwrap(), 42);    // defined symbol
/// assert!(table.get("undefined").is_err());       // undefined symbol
/// assert!(table.get("VALUE").is_err());           // undefined because the table is case sensitive
/// ```
///
/// ## Predefined Symbols
/// By design of the Hack assembly language we already have predefined symbols inside
/// of our symbol table.
/// ```
/// # use hack_asm::SymbolTable;
/// let mut table = SymbolTable::new();
///
/// assert!(table.set("R10", 101).is_err());           // built in symbol cannot be redefined
/// assert_eq!(table.get("SCREEN").unwrap(), 16384);   // predefined
/// assert_eq!(table.get("R10").unwrap(), 10);         // predefined
/// ```
pub struct SymbolTable {
    table: HashMap<String, HackInt>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::default(),
        }
    }

    /// Sets the value of a symbol to the specified value.
    /// This function should also create the symbol if it does not exist.
    /// Overwriting a built in or an already defined symbol is not allowed.
    /// May return a [`SymbolTableError`]
    /// # Arguments
    /// * `name` - A string that contains the name of the symbol
    /// * `value` - The value (or address) associated with the symbol
    pub fn set(&mut self, name: &str, value: HackInt) -> Result<(), SymbolTableError> {
        if BUILT_IN.get(name).is_some() {
            return Err(SymbolTableError::RedefinedBuiltIn(name.to_string()));
        }

        let entry = self.table.raw_entry_mut().from_key(name);
        match entry {
            std::collections::hash_map::RawEntryMut::Occupied(_) => {
                Err(SymbolTableError::Redefined(name.to_string()))
            }
            std::collections::hash_map::RawEntryMut::Vacant(entry) => {
                entry.insert(name.to_string(), value);
                Ok(())
            }
        }
    }

    /// Retrieves the value of a symbol.
    /// May return a [`SymbolTableError`]
    /// # Arguments
    /// * `name` - The symbol name to look up
    pub fn get(&self, name: &str) -> Result<HackInt, SymbolTableError> {
        if let Some(&built_in) = BUILT_IN.get(name) {
            return Ok(built_in);
        }

        if let Some(&user_defined) = self.table.get(name) {
            return Ok(user_defined);
        }

        Err(SymbolTableError::NotDefined(name.to_string()))
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_redefine_built_in() {
        let mut table = SymbolTable::new();

        // try to redefine a built in symbol
        let error = table.set("R1", 42).unwrap_err();

        match error {
            SymbolTableError::RedefinedBuiltIn(_) => (),
            _ => panic!("expected RedefinedBuiltIn error"),
        }
    }

    #[test]
    fn test_error_redefine_user_defined() {
        let mut table = SymbolTable::new();

        // try to redefine a user defined symbol
        table.set("some_var", 21).unwrap();
        let error = table.set("some_var", 42).unwrap_err();

        match error {
            SymbolTableError::Redefined(_) => (),
            _ => panic!("expected RedefinedBuiltIn error"),
        }
    }
}
