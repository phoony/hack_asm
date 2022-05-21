extern crate pest;

use pest::Parser;

use crate::instructions::Label;

use super::{
    a_instruction::a_instruction, c_instruction::c_instruction, label::label, ParseError,
    ParsedInstruction,
};

#[derive(Parser)]
#[grammar = "grammar/hack.pest"]
pub struct HackParser;

pub struct ParserOutput<'a> {
    pub instructions: Vec<ParsedInstruction<'a>>,
    pub labels: Vec<(Label<'a>, usize)>,
}

pub fn parse_str(input: &str) -> Result<ParserOutput, ParseError> {
    let mut program = HackParser::parse(Rule::program, input)?;
    let program = program.next().unwrap();
    let mut instructions = Vec::new();
    let mut labels = Vec::new();

    for instruction in program.into_inner() {
        match instruction.as_rule() {
            Rule::at_instruction => instructions.push(a_instruction(instruction)?),
            Rule::c_instruction => instructions.push(c_instruction(instruction)),
            Rule::label => labels.push((label(instruction), instructions.len())),
            Rule::EOI => (),
            _ => unreachable!(),
        };
    }

    Ok(ParserOutput {
        instructions,
        labels,
    })
}
