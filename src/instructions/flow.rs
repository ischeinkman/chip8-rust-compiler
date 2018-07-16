use instructions::*;

#[derive(Clone, Debug)]
pub struct Return {}

impl InstructionOps for Return {
    fn to_opcode(&self) -> u16 {
        0x00EE
    }

    fn parse_args(ln : &str) -> Result<Return, ParseError> {
        if !ln.trim().is_empty() && !ln.trim().starts_with("//") {
            Err(ParseError("Return takes no arguments.".to_owned()))
        }
        else {
            Ok(Return {})
        }
    }
}

#[derive(Clone, Debug)]
pub struct Jump (OpParam, OpParam);

impl InstructionOps for Jump {
    fn to_opcode(&self) -> u16 {
        match self {
            Jump(OpParam::Variable(addr), OpParam::Blank) => 0x1000 | (addr & 0x0FFF),
            Jump(OpParam::Label(ref lbl), OpParam::Blank) => panic!("Label {} not correclty replaced!", lbl),
            Jump(OpParam::Register(0), OpParam::Variable(addr)) => 0xB000 | (addr & 0x0FFF),
            _ => panic!("Could not correclty parse Jump.")
        }
    }

    fn parse_args(ln: &str) -> Result<Jump, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(0), &OpParam::Label(_))    |
            (&OpParam::Register(0), &OpParam::Variable(_)) |
            (&OpParam::Label(_), &OpParam::Blank)          | 
            (&OpParam::Variable(_), &OpParam::Blank) => Ok(Jump(parsed_dest, parsed_source)),
            _ => Err(ParseError("Got invalid Jump dest!".to_owned()))
        }
    }
}

impl InstructionOpsWithLabels for Jump {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Jump {
        let new_left = labels.get(&self.0).unwrap_or(&self.0);
        let new_right = labels.get(&self.1).unwrap_or(&self.1);
        Jump(new_left.clone(), new_right.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Call (OpParam);

impl InstructionOps for Call {
    fn to_opcode(&self) -> u16 {
        match self {
            &Call(OpParam::Variable(addr)) => 0x2000 | (addr & 0x0FFF),
            &Call(OpParam::Label(ref lbl)) => panic!("Label {} not correclty replaced!", lbl),
            _ => panic!("Could not correclty parse Call.")
        }
    }

    fn parse_args(ln: &str) -> Result<Call, ParseError> {
        let parsed_dest = parse_args!(ln, 1);

        match parsed_dest {
            OpParam::Label(_) | OpParam::Variable(_) => Ok(Call(parsed_dest)),
            _ => Err(ParseError("Got invalid Call dest!".to_owned()))
        }
    }
}

impl InstructionOpsWithLabels for Call {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Call {
        let new_left = labels.get(&self.0).unwrap_or(&self.0);
        Call(new_left.clone())
    }
}

#[derive(Clone, Debug)]
pub struct SkipIfEqual(OpParam, OpParam);

impl InstructionOps for SkipIfEqual {
    fn to_opcode(&self) -> u16 {
        match *self {
            SkipIfEqual(OpParam::Register(dreg), OpParam::Variable(vl)) => 0x3000 | (dreg as u16 & 0xF) << 8 | (vl & 0xFF),
            SkipIfEqual(OpParam::Register(dreg), OpParam::Register(sreg)) => 0x5000 | (dreg as u16 & 0xF) << 8  | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid SkipIfEqual parameter!")
        }
    }

    
    fn parse_args(ln: &str) -> Result<SkipIfEqual, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) | 
            (&OpParam::Register(_), &OpParam::Variable(_)) => { Ok(SkipIfEqual(parsed_dest, parsed_source)) },
            _ => Err(ParseError(format!("Could not parse load args: {}", ln)))
        }
    }
}

#[derive(Clone, Debug)]
pub struct SkipIfNotEqual(OpParam, OpParam);

impl InstructionOps for SkipIfNotEqual {
    fn to_opcode(&self) -> u16 {
        match *self {
            SkipIfNotEqual(OpParam::Register(dreg), OpParam::Variable(vl)) => 0x4000 | (dreg as u16 & 0xF) << 8 | (vl & 0xFF),
            SkipIfNotEqual(OpParam::Register(dreg), OpParam::Register(sreg)) => 0x9000 | (dreg as u16 & 0xF) << 8  | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid SkipIfEqual parameter!")
        }
    }

    
    fn parse_args(ln: &str) -> Result<SkipIfNotEqual, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) | 
            (&OpParam::Register(_), &OpParam::Variable(_)) => { Ok(SkipIfNotEqual(parsed_dest, parsed_source)) },
            _ => Err(ParseError(format!("Could not parse load args: {}", ln)))
        }
    }
}

#[derive(Clone, Debug)]
pub struct SkipIfKey (OpParam);

impl InstructionOps for SkipIfKey {
    fn parse_args(ln : &str) -> Result<SkipIfKey, ParseError> {
        let parsed_reg = parse_args!(ln, 1);
        match parsed_reg {
            OpParam::Register(_) => Ok(SkipIfKey(parsed_reg)),
            _ => Err(ParseError(format!("Could not parse SkipIfKey args: {}", ln)))
        }
    }

    fn to_opcode(&self) -> u16 {
        match *self {
            SkipIfKey(OpParam::Register(reg)) => 0xE09E | (reg as u16 & 0xF) << 8,
            _ => panic!("Got invalid SkipIfKey parameter!")
        }
    }
}

#[derive(Clone, Debug)]
pub struct SkipIfNotKey (OpParam);

impl InstructionOps for SkipIfNotKey {
    fn parse_args(ln : &str) -> Result<SkipIfNotKey, ParseError> {
        let parsed_reg = parse_args!(ln, 1);
        match parsed_reg {
            OpParam::Register(_) => Ok(SkipIfNotKey(parsed_reg)),
            _ => Err(ParseError(format!("Could not parse SkipIfNotKey args: {}", ln)))
        }
    }

    fn to_opcode(&self) -> u16 {
        match *self {
            SkipIfNotKey(OpParam::Register(reg)) => 0xE0A1 | (reg as u16 & 0xF) << 8,
            _ => panic!("Got invalid SkipIfNotKey parameter!")
        }
    }
}