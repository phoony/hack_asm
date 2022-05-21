use pest::iterators::Pair;

use crate::{
    hack_int::HackInt,
    instructions::{AInstruction, AValue},
};

use super::{ParseError, ParsedInstruction, Rule};

pub fn a_instruction(instruction: Pair<Rule>) -> Result<ParsedInstruction, ParseError> {
    let inner = instruction.into_inner().next().unwrap();

    let value = match inner.as_rule() {
        Rule::symbol => AValue::Symbol(inner.as_str()),
        Rule::literal => AValue::Literal(HackInt::parse(inner.as_str())?),
        _ => unreachable!(),
    };

    Ok(ParsedInstruction::AInstruction(AInstruction { value }))
}
