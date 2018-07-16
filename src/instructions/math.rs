#[macro_use]
use instructions::*;

#[derive(Clone, Debug)]
pub struct Add {
    acc : OpParam, 
    toAdd : OpParam
}

impl InstructionOps for Add {
    fn to_opcode(&self) -> u16 {
        match *self {
            Add{acc : OpParam::Register(dreg), toAdd : OpParam::Variable(vl)} => 0x7000 | ((dreg as u16 & 0xF) << 8) | vl & 0xFF, 
            Add{acc : OpParam::Register(dreg), toAdd : OpParam::Register(sreg)} => 0x8004 | ((dreg as u16 & 0xF) << 8) | ((sreg as u16 &0xF) << 4),
            Add{acc : OpParam::RegisterI, toAdd : OpParam::Register(sreg)} => 0xF01E | ((sreg as u16 & 0xF)) << 8,  
            _ => panic!("Got invalid Add parameter!")
        }
    }

    fn parse_args(ln : &str) -> Result<Add, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::RegisterI, &OpParam::Register(_))   |
            (&OpParam::Register(_), &OpParam::Register(_)) | 
            (&OpParam::Register(_), &OpParam::Variable(_)) => { 
                Ok(Add{
                    acc: parsed_dest, 
                    toAdd: parsed_source
                }) 
            },
            _ => Err(ParseError(format!("Could not parse load args: {}", ln)))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sub {acc : OpParam, reg : OpParam}

impl InstructionOps for Sub {
    fn to_opcode(&self) -> u16 {
        match *self {
            Sub{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8005 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid Sub parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<Sub, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) => { Ok(Sub{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse Sub args: {}", ln)))
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubN {acc : OpParam, reg : OpParam}

impl InstructionOps for SubN {
    fn to_opcode(&self) -> u16 {
        match *self {
            SubN{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8007 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid SubN parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<SubN, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) => { Ok(SubN{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse SubN args: {}", ln)))
        }
    }
}