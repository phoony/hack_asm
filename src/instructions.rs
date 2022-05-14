use crate::{HackInt, SYMBOL_TABLE};
use anyhow::{bail, Result};
use phf::phf_map;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("invalid destination: \"{0}\"")]
    Destination(String),
    #[error("invalid computation: \"{0}\"")]
    Computation(String),
    #[error("invalid jump instruction: \"{0}\"")]
    Jump(String),
}

pub trait Compilable {
    fn compile(self) -> Result<u16, anyhow::Error>;
}

pub enum AInstruction<'a> {
    Immediate(HackInt),
    Symbol(&'a str),
}

impl<'a> Compilable for AInstruction<'a> {
    fn compile(self) -> Result<u16, anyhow::Error> {
        match self {
            AInstruction::Immediate(val) => Ok(val),
            AInstruction::Symbol(name) => {
                let symbol_table = &SYMBOL_TABLE;
                match symbol_table.read() {
                    Ok(table) => Ok(table.get(name)?),
                    Err(e) => {
                        let table = e.into_inner();
                        Ok(table.get(name)?)
                    }
                }
            }
        }
    }
}

pub struct CInstruction<'a> {
    destination: Option<&'a str>,
    computation: &'a str,
    jump: Option<&'a str>,
}

impl<'a> CInstruction<'a> {
    const DEST_TABLE: phf::Map<&'static str, u16> = phf_map! {
        "M"   => 0b0000000000_001_000,
        "D"   => 0b0000000000_010_000,
        "MD"  => 0b0000000000_011_000,
        "A"   => 0b0000000000_100_000,
        "AM"  => 0b0000000000_101_000,
        "AD"  => 0b0000000000_110_000,
        "AMD" => 0b0000000000_111_000,
    };

    const COMP_TABLE: phf::Map<&'static str, u16> = phf_map! {
        "0"   => 0b000_0101010_000000,
        "1"   => 0b000_0111111_000000,
        "-1"  => 0b000_0111010_000000,
        "D"   => 0b000_0001100_000000,
        "A"   => 0b000_0110000_000000,
        "!D"  => 0b000_0001101_000000,
        "!A"  => 0b000_0110001_000000,
        "-D"  => 0b000_0001111_000000,
        "-A"  => 0b000_0110011_000000,
        "D+1" => 0b000_0011111_000000,
        "A+1" => 0b000_0110111_000000,
        "D-1" => 0b000_0001110_000000,
        "A-1" => 0b000_0110010_000000,
        "D+A" => 0b000_0000010_000000,
        "D-A" => 0b000_0010011_000000,
        "A-D" => 0b000_0000111_000000,
        "D&A" => 0b000_0000000_000000,
        "D|A" => 0b000_0010101_000000,
        "M"   => 0b000_1110000_000000,
        "!M"  => 0b000_1110001_000000,
        "-M"  => 0b000_1110011_000000,
        "M+1" => 0b000_1110111_000000,
        "M-1" => 0b000_1110010_000000,
        "D+M" => 0b000_1000010_000000,
        "D-M" => 0b000_1010011_000000,
        "M-D" => 0b000_1000111_000000,
        "D&M" => 0b000_1000000_000000,
        "D|M" => 0b000_1010101_000000,
    };

    const JMP_TABLE: phf::Map<&'static str, u16> = phf_map! {
        "JGT" => 0b000000000000_001,
        "JEQ" => 0b000000000000_010,
        "JGE" => 0b000000000000_011,
        "JLT" => 0b000000000000_100,
        "JNE" => 0b000000000000_101,
        "JLE" => 0b000000000000_110,
        "JMP" => 0b000000000000_111,
    };
}

impl<'a> Compilable for CInstruction<'a> {
    fn compile(self) -> Result<u16, anyhow::Error> {
        let mut instruction = 0b1110000000000000;

        // Lookup destination bits
        if let Some(dest) = self.destination {
            match Self::DEST_TABLE.get(dest) {
                Some(bits) => instruction |= bits,
                None => bail!(CompilationError::Destination(dest.to_string())),
            }
        }

        // Lookup computation bits
        match Self::COMP_TABLE.get(self.computation) {
            Some(bits) => instruction |= bits,
            None => bail!(CompilationError::Computation(self.computation.to_string())),
        }

        // Lookup jump bits
        if let Some(jump) = self.jump {
            match Self::JMP_TABLE.get(jump) {
                Some(bits) => instruction |= bits,
                None => bail!(CompilationError::Jump(jump.to_string())),
            }
        }

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod a_instruction {
        use super::*;

        #[test]
        fn immediate_at_42() -> Result<(), anyhow::Error> {
            let instr = AInstruction::Immediate(42);
            assert_eq!(instr.compile()?, 0b0000000000101010);
            Ok(())
        }

        #[test]
        fn immediate_at_0() -> Result<(), anyhow::Error> {
            let instr = AInstruction::Immediate(0);
            assert_eq!(instr.compile()?, 0b0);
            Ok(())
        }

        #[test]
        fn immediate_at_32767() -> Result<(), anyhow::Error> {
            let instr = AInstruction::Immediate(32767);
            assert_eq!(instr.compile()?, 0b0111111111111111);
            Ok(())
        }
    }

    mod c_instruction {
        use super::*;

        fn test_c(dest: &str, comp: &str, jump: &str, equals: u16) -> bool {
            fn to_option(s: &str) -> Option<&str> {
                match s {
                    "" => None,
                    other => Some(other),
                }
            }

            let instr = CInstruction {
                destination: to_option(dest),
                computation: comp,
                jump: to_option(jump),
            };

            instr.compile().unwrap() == equals
        }

        #[test]
        fn complete() {
            assert!(test_c("AMD", "D+M", "JMP", 0b1111000010111111));
            assert!(test_c("D", "A", "", 0b1110110000010000));
            assert!(test_c("M", "D|M", "", 0b1111010101001000));
            assert!(test_c("A", "A-1", "", 0b1110110010100000));
            assert!(test_c("", "0", "JMP", 0b1110101010000111));
            assert!(test_c("", "D", "JLE", 0b1110001100000110));
            assert!(test_c("AM", "M+1", "", 0b1111110111101000));
            assert!(test_c("D", "D&A", "", 0b1110000000010000));
            assert!(test_c("D", "M", "", 0b1111110000010000));
            assert!(test_c("D", "!M", "", 0b1111110001010000));
        }
    }
}
