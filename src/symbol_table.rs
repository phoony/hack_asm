use phf::phf_map;
use std::collections::HashMap;
use thiserror::Error;

use crate::hack_int::{HackInt, ParseHackIntError};

#[derive(Error, Debug)]
pub enum SymbolTableGetError {
    #[error("symbol \"{0}\" not defined")]
    NotDefined(String),
}

#[derive(Error, Debug)]
pub enum SymbolTableSetError {
    #[error("tried to redefine the built in symbol \"{0}\"")]
    RedefinedBuiltIn(String),
    #[error("tried to redefine the symbol \"{0}\"")]
    Redefined(String),
}

static BUILT_IN: phf::Map<&'static str, HackInt> = phf_map! {
    // Virtual Registers
    "R0" =>  HackInt::new_unchecked(0),
    "R1" =>  HackInt::new_unchecked(1),
    "R2" =>  HackInt::new_unchecked(2),
    "R3" =>  HackInt::new_unchecked(3),
    "R4" =>  HackInt::new_unchecked(4),
    "R5" =>  HackInt::new_unchecked(5),
    "R6" =>  HackInt::new_unchecked(6),
    "R7" =>  HackInt::new_unchecked(7),
    "R8" =>  HackInt::new_unchecked(8),
    "R9" =>  HackInt::new_unchecked(9),
    "R10" => HackInt::new_unchecked(10),
    "R11" => HackInt::new_unchecked(11),
    "R12" => HackInt::new_unchecked(12),
    "R13" => HackInt::new_unchecked(13),
    "R14" => HackInt::new_unchecked(14),
    "R15" => HackInt::new_unchecked(15),
    // IO
    "SCREEN" => HackInt::new_unchecked(16384),
    "KBD"    => HackInt::new_unchecked(24576),
    // Reserved
    "SP"   => HackInt::new_unchecked(0),
    "LCL"  => HackInt::new_unchecked(1),
    "ARG"  => HackInt::new_unchecked(2),
    "THIS" => HackInt::new_unchecked(3),
    "THAT" => HackInt::new_unchecked(4),
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
/// ```ignore
/// # use hack_asm::SymbolTable;
/// let mut table = SymbolTable::new();
///
/// assert!(table.set("R10", 101).is_err());           // built in symbol cannot be redefined
/// assert_eq!(table.get("SCREEN").unwrap(), 16384);   // predefined
/// assert_eq!(table.get("R10").unwrap(), 10);         // predefined
/// ```
pub struct SymbolTable {
    table: HashMap<String, HackInt>,
    variable_index: HackInt,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::default(),
            variable_index: HackInt::new_unchecked(16),
        }
    }

    /// Sets the value of a symbol to the specified value.
    /// This function should also create the symbol if it does not exist.
    /// Overwriting a built in or an already defined symbol is not allowed.
    /// May return a [`SymbolTableError`]
    /// # Arguments
    /// * `name` - A string that contains the name of the symbol
    /// * `value` - The value (or address) associated with the symbol
    pub fn set(&mut self, name: &str, value: HackInt) -> Result<(), SymbolTableSetError> {
        if BUILT_IN.get(name).is_some() {
            return Err(SymbolTableSetError::RedefinedBuiltIn(name.to_string()));
        }

        let entry = self.table.raw_entry_mut().from_key(name);
        match entry {
            std::collections::hash_map::RawEntryMut::Occupied(_) => {
                Err(SymbolTableSetError::Redefined(name.to_string()))
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
    pub fn get(&self, name: &str) -> Result<HackInt, SymbolTableGetError> {
        if let Some(&built_in) = BUILT_IN.get(name) {
            return Ok(built_in);
        }

        if let Some(&user_defined) = self.table.get(name) {
            return Ok(user_defined);
        }

        Err(SymbolTableGetError::NotDefined(name.to_string()))
    }

    pub fn get_variable_index(&mut self) -> HackInt {
        self.variable_index
    }

    pub fn increment_variable_index(&mut self) -> Result<(), ParseHackIntError> {
        let index: u16 = self.variable_index.into();
        self.variable_index = HackInt::try_new(index + 1)?;
        Ok(())
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
        let error = table.set("R1", HackInt::new_unchecked(42)).unwrap_err();

        match error {
            SymbolTableSetError::RedefinedBuiltIn(_) => (),
            _ => panic!("expected RedefinedBuiltIn error"),
        }
    }

    #[test]
    fn test_error_redefine_user_defined() {
        let mut table = SymbolTable::new();

        // try to redefine a user defined symbol
        table.set("some_var", HackInt::new_unchecked(42)).unwrap();
        let error = table
            .set("some_var", HackInt::new_unchecked(42))
            .unwrap_err();

        match error {
            SymbolTableSetError::Redefined(_) => (),
            _ => panic!("expected RedefinedBuiltIn error"),
        }
    }
}
