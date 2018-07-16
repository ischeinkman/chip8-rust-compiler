use instructions::*;

#[derive(Clone, Debug)]
pub struct Or {acc : OpParam, reg : OpParam}

impl InstructionOps for Or {
    fn to_opcode(&self) -> u16 {
        match *self {
            Or{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8001 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid Or parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<Or, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) => { Ok(Or{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse Or args: {}", ln)))
        }
    }
}

#[derive(Clone, Debug)]
pub struct And {acc : OpParam, reg : OpParam}

impl InstructionOps for And {
    fn to_opcode(&self) -> u16 {
        match *self {
            And{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8002 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid And parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<And, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) => { Ok(And{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse And args: {}", ln)))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Xor {acc : OpParam, reg : OpParam}

impl InstructionOps for Xor {
    fn to_opcode(&self) -> u16 {
        match *self {
            Xor{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8003 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid Xor parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<Xor, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) => { Ok(Xor{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse Xor args: {}", ln)))
        }
    }
}


#[derive(Clone, Debug)]
pub struct ShiftRight {
    acc : OpParam, 
    usually_unused : OpParam
}

impl InstructionOps for ShiftRight {
    fn parse_args(ln : &str) -> Result<ShiftRight, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) => { Ok(ShiftRight{acc: parsed_dest, usually_unused: parsed_source}) },
            (&OpParam::Register(_), &OpParam::Blank) => { Ok(ShiftRight{acc: parsed_dest.clone(), usually_unused: parsed_dest}) },
            _ => Err(ParseError(format!("Could not parse ShiftRight args: {}", ln)))
        }

    }

    fn to_opcode(&self) -> u16 {
        match *self {
            ShiftRight{acc : OpParam::Register(dreg), usually_unused : OpParam::Register(oreg)} => 0x8006 | (dreg as u16 & 0xF) << 8 | (oreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid ShiftRight parameter!")
        }
    }
}


#[derive(Clone, Debug)]
pub struct ShiftLeft {
    acc : OpParam, 
    usually_unused : OpParam
}

impl InstructionOps for ShiftLeft {
    fn parse_args(ln : &str) -> Result<ShiftLeft, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) => { Ok(ShiftLeft{acc: parsed_dest, usually_unused: parsed_source}) },
            (&OpParam::Register(_), &OpParam::Blank) => { Ok(ShiftLeft{acc: parsed_dest.clone(), usually_unused: parsed_dest}) },
            _ => Err(ParseError(format!("Could not parse ShiftLeft args: {}", ln)))
        }

    }

    fn to_opcode(&self) -> u16 {
        match *self {
            ShiftLeft{acc : OpParam::Register(dreg), usually_unused : OpParam::Register(oreg)} => 0x800E | (dreg as u16 & 0xF) << 8 | (oreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid ShiftLeft parameter!")
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rand {
    reg : OpParam,
    mask : OpParam
}

impl InstructionOps for Rand {
    fn parse_args(ln : &str) -> Result<Rand, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Variable(_)) => { Ok(Rand{reg: parsed_dest, mask: parsed_source}) },
            (&OpParam::Register(_), &OpParam::Blank) => { Ok(Rand{reg: parsed_dest, mask: OpParam::Variable(0x00FF)}) },
            _ => Err(ParseError(format!("Could not parse Rand args: {}", ln)))
        }

    }

    fn to_opcode(&self) -> u16 {
        match *self {
            Rand{reg : OpParam::Register(dreg), mask : OpParam::Variable(msk)} => 0xC000 | (dreg as u16 & 0xF) << 8 | (msk as u16 & 0xFF),
            _ => panic!("Got invalid Rand parameter!")
        }
    }
}

