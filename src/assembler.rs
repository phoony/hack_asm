use crate::{
    assembler_context::{AssemblerContext, AssemblerError},
    parsing::parse_str,
};

pub struct Assembler<'a> {
    context: AssemblerContext,
    input: &'a str,
}

impl<'a> Assembler<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            context: AssemblerContext::default(),
            input,
        }
    }

    pub fn assemble(mut self) -> Result<Vec<u16>, AssemblerError> {
        let parser_output = parse_str(self.input)?;

        for (label, index) in parser_output.labels {
            self.context.register_label(label, index)?;
        }

        for instruction in parser_output.instructions {
            self.context.feed_instruction(instruction)?;
        }

        Ok(self.context.into_output())
    }
}
