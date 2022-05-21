use thiserror::Error;

use crate::{
    hack_int::HackInt,
    instructions::Label,
    parsing::ParsedInstruction,
    symbol_table::{SymbolTable, SymbolTableGetError, SymbolTableSetError},
};

pub struct AssemblerContext {
    symbol_table: SymbolTable,
    current_variable_address: HackInt,
    current_label_address: HackInt,
    pub output: Vec<u16>,
}

#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("exceeded maximum number of variables")]
    TooManyVariables,
    #[error("exceeded maximum number of instructions")]
    TooManyInstructions,
    #[error(transparent)]
    SymbolTableSetError(#[from] SymbolTableSetError),
}

impl AssemblerContext {
    fn set_symbol(&mut self, name: &str, value: HackInt) -> Result<(), SymbolTableSetError> {
        self.symbol_table.set(name, value)?;
        Ok(())
    }

    fn get_symbol(&self, name: &str) -> Result<HackInt, SymbolTableGetError> {
        self.symbol_table.get(name)
    }

    pub fn register_label(&mut self, label: Label, address: usize) -> Result<(), AssemblerError> {
        let address = HackInt::new_unchecked(address as u16);
        self.symbol_table.set(label.name, address)?;

        Ok(())
    }

    fn push_instruction(&mut self, bits: u16) -> Result<(), AssemblerError> {
        if self.output.len() >= crate::constants::ROM_SIZE {
            return Err(AssemblerError::TooManyInstructions);
        }

        self.output.push(bits);
        self.current_label_address.inc_unchecked();

        Ok(())
    }

    pub fn feed_instruction(&mut self, instr: ParsedInstruction) -> Result<(), AssemblerError> {
        match instr {
            ParsedInstruction::AInstruction(i) => {
                let bits = i.to_u16(self)?;
                self.push_instruction(bits)
            }
            ParsedInstruction::CInstruction(i) => self.push_instruction(i.to_u16()),
        }
    }

    pub fn get_or_create_variable(&mut self, name: &str) -> Result<u16, AssemblerError> {
        if let Ok(value) = self.get_symbol(name) {
            return Ok(value.into());
        }

        if self.current_variable_address >= crate::constants::MEMORY_SIZE {
            return Err(AssemblerError::TooManyVariables);
        }

        self.set_symbol(name, self.current_variable_address)?;
        let result = self.current_variable_address.into();
        self.current_variable_address.inc_unchecked();

        Ok(result)
    }
}

impl Default for AssemblerContext {
    fn default() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            current_variable_address: HackInt::new_unchecked(16),
            current_label_address: HackInt::new_unchecked(0),
            output: Vec::new(),
        }
    }
}
