use std::collections::HashMap;

pub mod parameters;
#[macro_use]
pub mod macros;
pub mod bitops;
pub mod display;
pub mod flow;
pub mod loads;
pub mod math;

use bitops::*;
use display::*;
use flow::*;
use loads::*;
use math::*;
use parameters::*;

#[derive(Debug)]
pub struct ParseError(String);

pub trait InstructionOps: Sized {
    fn to_opcode(&self) -> u16;

    fn parse_args(ln: &str) -> Result<Self, ParseError>;
}

pub trait InstructionOpsWithLabels: InstructionOps {
    fn resolve_labels(&self, labels: &HashMap<OpParam, OpParam>) -> Self;
}

#[derive(Clone)]
pub enum Instruction {
    Jump(flow::Jump),
    Call(flow::Call),
    Return(flow::Return),

    SkipIfEqual(flow::SkipIfEqual),
    SkipIfNotEqual(flow::SkipIfNotEqual),
    SkipIfKey(flow::SkipIfKey),
    SkipIfNotKey(flow::SkipIfNotKey),

    Load(loads::Load),

    And(bitops::And),
    Or(bitops::Or),
    Xor(bitops::Xor),
    Rand(bitops::Rand),
    ShiftLeft(bitops::ShiftLeft),
    ShiftRight(bitops::ShiftRight),

    ClearScreen(display::ClearScreen),
    Draw(display::Draw),

    Add(math::Add),
    Sub(math::Sub),
    SubN(math::SubN),
}

impl InstructionOps for Instruction {
    fn to_opcode(&self) -> u16 {
        match self {
            Instruction::Jump(obj) => obj.to_opcode(),
            Instruction::Call(obj) => obj.to_opcode(),
            Instruction::Return(obj) => obj.to_opcode(),

            Instruction::SkipIfEqual(obj) => obj.to_opcode(),
            Instruction::SkipIfNotEqual(obj) => obj.to_opcode(),
            Instruction::SkipIfKey(obj) => obj.to_opcode(),
            Instruction::SkipIfNotKey(obj) => obj.to_opcode(),

            Instruction::Load(obj) => obj.to_opcode(),

            Instruction::And(obj) => obj.to_opcode(),
            Instruction::Or(obj) => obj.to_opcode(),
            Instruction::Xor(obj) => obj.to_opcode(),
            Instruction::Rand(obj) => obj.to_opcode(),
            Instruction::ShiftLeft(obj) => obj.to_opcode(),
            Instruction::ShiftRight(obj) => obj.to_opcode(),

            Instruction::ClearScreen(obj) => obj.to_opcode(),
            Instruction::Draw(obj) => obj.to_opcode(),

            Instruction::Add(obj) => obj.to_opcode(),
            Instruction::Sub(obj) => obj.to_opcode(),
            Instruction::SubN(obj) => obj.to_opcode(),
        }
    }

    fn parse_args(ln : &str) -> Result<Instruction, ParseError> {
        let true_line = ln.splitn(2, "//").next().unwrap_or("").trim().to_uppercase();

        let mut tln_itr = true_line.splitn(2, " ");
        let instr = tln_itr.next().unwrap_or("").trim();
        let args = tln_itr.next().unwrap_or("").trim();

        match instr {
            "JP" => Jump::parse_args(args).map(|inner| Instruction::Jump(inner)),
            "CALL" => Call::parse_args(args).map(|inner| Instruction::Call(inner)),
            "RET" => Return::parse_args(args).map(|inner| Instruction::Return(inner)),
            
            "SE" => SkipIfEqual::parse_args(args).map(|inner| Instruction::SkipIfEqual(inner)),
            "SNE" => SkipIfNotEqual::parse_args(args).map(|inner| Instruction::SkipIfNotEqual(inner)),
            "SKP" => SkipIfKey::parse_args(args).map(|inner| Instruction::SkipIfKey(inner)),
            "SKNP" => SkipIfNotKey::parse_args(args).map(|inner| Instruction::SkipIfNotKey(inner)),
            
            "LD" => Load::parse_args(args).map(|inner| Instruction::Load(inner)),
            
            "AND" => And::parse_args(args).map(|inner| Instruction::And(inner)),
            "OR" => Or::parse_args(args).map(|inner| Instruction::Or(inner)),
            "XOR" => Xor::parse_args(args).map(|inner| Instruction::Xor(inner)),
            "RAND" => Rand::parse_args(args).map(|inner| Instruction::Rand(inner)),
            "SHL" => ShiftLeft::parse_args(args).map(|inner| Instruction::ShiftLeft(inner)),
            "SHR" => ShiftRight::parse_args(args).map(|inner| Instruction::ShiftRight(inner)),

            "CLS" => ClearScreen::parse_args(args).map(|inner| Instruction::ClearScreen(inner)),
            "DRW" => Draw::parse_args(args).map(|inner| Instruction::Draw(inner)),
            
            "ADD" => Add::parse_args(args).map(|inner| Instruction::Add(inner)),
            "SUB" => Sub::parse_args(args).map(|inner| Instruction::Sub(inner)),
            "SUBN" => SubN::parse_args(args).map(|inner| Instruction::SubN(inner)),


            _ => Err(ParseError(format!("Could not parse instruction from line: {}", ln)))
        }

    }
}

impl InstructionOpsWithLabels for Instruction {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Instruction {
        match self {
            Instruction::Jump(obj) => Instruction::Jump(obj.resolve_labels(labels)),
            Instruction::Call(obj) => Instruction::Call(obj.resolve_labels(labels)), 
            Instruction::Load(obj) => Instruction::Load(obj.resolve_labels(labels)),
            _ => self.clone()
        }
    }
}

pub fn is_instr_line(ln: &str) -> bool {
    let tln = ln.trim().to_uppercase();

    tln.starts_with("CLS")
        || tln.starts_with("RET")
        || tln.starts_with("JP")
        || tln.starts_with("CALL")
        || tln.starts_with("SE")
        || tln.starts_with("SNE")
        || tln.starts_with("LD")
        || tln.starts_with("OR")
        || tln.starts_with("AND")
        || tln.starts_with("XOR")
        || tln.starts_with("ADD")
        || tln.starts_with("SUB")
        || tln.starts_with("SHR")
        || tln.starts_with("SUBN")
        || tln.starts_with("SHL")
        || tln.starts_with("RND")
        || tln.starts_with("DRW")
        || tln.starts_with("SKP")
        || tln.starts_with("SKNP")
}

pub fn is_label(ln: &str) -> bool {
    !ln.starts_with("//") && !ln.trim().is_empty() && !is_instr_line(ln)
}

pub fn parse_labels(file: &str) -> HashMap<OpParam, OpParam> {
    let mut map = HashMap::new();
    let mut offset: u16 = 0x200;

    for ln in file.lines() {
        if is_instr_line(ln) {
            offset += 2;
        } else if is_label(ln) {
            let comment_parsed = ln.splitn(2, "//").next().unwrap();
            let nlabel = OpParam::Label(comment_parsed.to_owned());
            let nvariable = OpParam::Variable(offset);
            map.insert(nlabel, nvariable);
        }
    }
    map
}
