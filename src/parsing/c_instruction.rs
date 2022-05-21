use pest::iterators::Pair;

use crate::instructions::{CInstruction, Computation, JumpType, Register};

use super::{ParsedInstruction, Rule};

fn c_dest(dest: Pair<Rule>) -> Vec<Register> {
    dest.into_inner()
        .map(
            |c| match c.as_str().chars().next().unwrap().to_ascii_uppercase() {
                'A' => Register::A,
                'M' => Register::M,
                'D' => Register::D,
                _ => unreachable!(),
            },
        )
        .collect()
}

fn c_comp(comp: Pair<Rule>) -> Computation {
    let comp = comp.into_inner().next().unwrap();

    match comp.as_rule() {
        Rule::constant => constant(comp),
        Rule::register => Computation::Identity(register(comp)),
        Rule::unary => unary(comp),
        Rule::binary => binary(comp),
        _ => unreachable!(),
    }
}

fn constant(constant: Pair<Rule>) -> Computation {
    let constant = constant.into_inner().next().unwrap();

    match constant.as_rule() {
        Rule::one => Computation::Literal(1),
        Rule::zero => Computation::Literal(0),
        Rule::neg_one => Computation::Literal(-1),
        _ => unreachable!(),
    }
}

fn register(register: Pair<Rule>) -> Register {
    match register
        .as_str()
        .chars()
        .next()
        .unwrap()
        .to_ascii_uppercase()
    {
        'A' => Register::A,
        'M' => Register::M,
        'D' => Register::D,
        _ => unreachable!(),
    }
}

fn unary(unary: Pair<Rule>) -> Computation {
    let mut parts: Vec<_> = unary.into_inner().collect();

    if parts[0].as_rule() == Rule::register {
        // Post Operator
        // such as D+1 or M-1
        match parts[1].as_rule() {
            Rule::inc => Computation::Inc(register(parts.remove(0))),
            Rule::dec => Computation::Dec(register(parts.remove(0))),
            _ => unreachable!(),
        }
    } else {
        // Pre Operator
        // such as !D or -M
        match parts[0].as_str() {
            "!" => Computation::Not(register(parts.remove(1))),
            "-" => Computation::Neg(register(parts.remove(1))),
            _ => unreachable!(),
        }
    }
}

fn binary(binary: Pair<Rule>) -> Computation {
    let mut parts: Vec<_> = binary.into_inner().collect();

    let reg2 = parts.pop().unwrap();
    let op = parts.pop().unwrap();
    let reg1 = parts.pop().unwrap();

    match op.as_str() {
        "+" => Computation::Add(register(reg1), register(reg2)),
        "-" => Computation::Sub(register(reg1), register(reg2)),
        "|" => Computation::Or(register(reg1), register(reg2)),
        "&" => Computation::And(register(reg1), register(reg2)),
        _ => unimplemented!(),
    }
}

fn c_jump(jump: Pair<Rule>) -> JumpType {
    match jump.as_str().to_ascii_uppercase().as_str() {
        "JMP" => JumpType::Jmp,
        "JGT" => JumpType::Jgt,
        "JEQ" => JumpType::Jeq,
        "JLT" => JumpType::Jlt,
        "JGE" => JumpType::Jge,
        "JLE" => JumpType::Jle,
        "JNE" => JumpType::Jne,
        _ => unreachable!(),
    }
}

pub fn c_instruction(instruction: Pair<Rule>) -> ParsedInstruction {
    let c_instr = instruction.into_inner();

    let mut destination: Option<Vec<Register>> = None;
    let mut computation: Computation = Computation::Literal(0);
    let mut jump: Option<JumpType> = None;

    for part in c_instr {
        match part.as_rule() {
            Rule::destination => destination = Some(c_dest(part)),
            Rule::computation => computation = c_comp(part),
            Rule::jump => jump = Some(c_jump(part)),
            _ => unreachable!(),
        }
    }

    ParsedInstruction::CInstruction(CInstruction {
        destination,
        computation,
        jump,
    })
}
