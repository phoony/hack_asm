#![allow(clippy::unusual_byte_groupings)]

use crate::{
    assembler_context::{AssemblerContext, AssemblerError},
    hack_int::HackInt,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    D,
    M,
    A,
}

#[derive(Debug, Clone, Copy)]
pub enum JumpType {
    Jmp,
    Jgt,
    Jeq,
    Jlt,
    Jge,
    Jle,
    Jne,
}

#[derive(Debug, Clone, Copy)]
pub enum Computation {
    Literal(i8),
    Identity(Register),
    Not(Register),
    Neg(Register),
    Inc(Register),
    Dec(Register),
    Add(Register, Register),
    Sub(Register, Register),
    And(Register, Register),
    Or(Register, Register),
}

#[derive(Debug)]
pub struct CInstruction {
    pub destination: Option<Vec<Register>>,
    pub computation: Computation,
    pub jump: Option<JumpType>,
}

impl CInstruction {
    fn jump_mask(jump: JumpType) -> u16 {
        match jump {
            JumpType::Jgt => 0b0000000000000_001,
            JumpType::Jeq => 0b0000000000000_010,
            JumpType::Jge => 0b0000000000000_011,
            JumpType::Jlt => 0b0000000000000_100,
            JumpType::Jne => 0b0000000000000_101,
            JumpType::Jle => 0b0000000000000_110,
            JumpType::Jmp => 0b0000000000000_111,
        }
    }

    fn register_mask(register: &Register) -> u16 {
        match register {
            Register::D => 0b0000000000_010_000,
            Register::M => 0b0000000000_001_000,
            Register::A => 0b0000000000_100_000,
        }
    }

    fn dest_mask(destinations: &[Register]) -> u16 {
        let mut dest = 0;

        for d in destinations {
            dest |= CInstruction::register_mask(d);
        }

        dest
    }

    fn computation_mask(computation: Computation) -> u16 {
        match computation {
            Computation::Literal(0) => 0b000_0101010_000000,
            Computation::Literal(1) => 0b000_0111111_000000,
            Computation::Literal(-1) => 0b000_0111010_000000,
            Computation::Identity(Register::D) => 0b000_0001100_000000,
            Computation::Identity(Register::A) => 0b000_0110000_000000,
            Computation::Identity(Register::M) => 0b000_1110000_000000,
            Computation::Not(Register::D) => 0b000_0001101_000000,
            Computation::Not(Register::A) => 0b000_0110001_000000,
            Computation::Not(Register::M) => 0b000_1110001_000000,
            Computation::Neg(Register::D) => 0b000_0001111_000000,
            Computation::Neg(Register::A) => 0b000_0110011_000000,
            Computation::Neg(Register::M) => 0b000_1110011_000000,
            Computation::Inc(Register::D) => 0b000_0011111_000000,
            Computation::Inc(Register::A) => 0b000_0110111_000000,
            Computation::Inc(Register::M) => 0b000_1110111_000000,
            Computation::Dec(Register::D) => 0b000_0001110_000000,
            Computation::Dec(Register::A) => 0b000_0110010_000000,
            Computation::Dec(Register::M) => 0b000_1110010_000000,
            Computation::Add(Register::D, Register::A)
            | Computation::Add(Register::A, Register::D) => 0b000_0000010_000000,
            Computation::Add(Register::D, Register::M)
            | Computation::Add(Register::M, Register::D) => 0b000_1000010_000000,
            Computation::Sub(Register::D, Register::A) => 0b000_0010011_000000,
            Computation::Sub(Register::A, Register::D) => 0b000_0000111_000000,
            Computation::Sub(Register::D, Register::M) => 0b000_1010011_000000,
            Computation::Sub(Register::M, Register::D) => 0b000_1000111_000000,
            Computation::And(Register::D, Register::A) => 0b000_0000000_000000,
            Computation::And(Register::D, Register::M) => 0b000_1000000_000000,
            Computation::Or(Register::D, Register::A) => 0b000_0010101_000000,
            Computation::Or(Register::D, Register::M) => 0b000_1010101_000000,
            _ => todo!(),
        }
    }
}

impl CInstruction {
    pub fn to_u16(&self) -> u16 {
        let mut instruction = 0b1110000000000000;

        if let Some(destination) = &self.destination {
            instruction |= CInstruction::dest_mask(destination)
        }

        if let Some(jump) = self.jump {
            instruction |= CInstruction::jump_mask(jump)
        }

        instruction |= CInstruction::computation_mask(self.computation);

        instruction
    }
}

pub enum AValue<'a> {
    Symbol(&'a str),
    Literal(HackInt),
}

pub struct AInstruction<'a> {
    pub value: AValue<'a>,
}

impl<'a> AInstruction<'a> {
    pub fn to_u16(&self, context: &mut AssemblerContext) -> Result<u16, AssemblerError> {
        match &self.value {
            AValue::Symbol(name) => Ok(context.get_or_create_variable(name)?),
            AValue::Literal(value) => Ok((*value).into()),
        }
    }
}

#[derive(Debug)]
pub struct Label<'a> {
    pub name: &'a str,
}
