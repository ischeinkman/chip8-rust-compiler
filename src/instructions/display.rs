use instructions::*;

#[derive(Clone, Debug)]
pub struct ClearScreen {}

impl InstructionOps for ClearScreen {
    fn to_opcode(&self) -> u16 {
        0x00E0
    }

    fn parse_args(ln : &str) -> Result<ClearScreen, ParseError> {
        if !ln.is_empty() && !ln.trim().starts_with("//") {
            Err(ParseError("Clear screen takes no arguments.".to_owned()))
        }
        else {
            Ok(ClearScreen {})
        }
    }
}

#[derive(Clone, Debug)]
pub struct Draw {
    xreg : OpParam,
    yreg : OpParam,
    length : OpParam
}

impl InstructionOps for Draw {
    fn parse_args(ln : &str) -> Result<Draw, ParseError> {
        let (parsed_dest, parsed_source, parsed_len) = parse_args!(ln, 3);

        match (&parsed_dest, &parsed_source, &parsed_len) {
            (&OpParam::Register(_), &OpParam::Register(_), &OpParam::Variable(_)) => { Ok(Draw{xreg: parsed_dest, yreg: parsed_source, length : parsed_len}) },
            _ => Err(ParseError(format!("Could not parse Draw args: {}", ln)))
        }

    }

    fn to_opcode(&self) -> u16 {
        match *self {
            Draw{xreg : OpParam::Register(x), yreg : OpParam::Register(y), length : OpParam::Variable(len)} => 0xD000 | (x as u16 & 0xF) << 8 | (y as u16 & 0xF) << 4 | (len & 0xF),
            _ => panic!("Got invalid Draw parameter!")
        }
    }
}