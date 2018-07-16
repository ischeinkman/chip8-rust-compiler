pub use std::collections::HashMap;

pub struct ParseError(String);

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum OpParam {
    Register(u8),

    Variable(u16),

    RegisterI,

    DerefI,

    Timer,

    AudioTimer,

    Keyboard,

    Blank,

    Label(String),

    Fontset, 

    Digits
}

impl OpParam {
    pub fn parse(instr: &str) -> OpParam {
        match instr {
            "K" => OpParam::Keyboard,
            "DT" => OpParam::Timer,
            "ST" => OpParam::AudioTimer,
            "I" => OpParam::RegisterI,
            "[I]" => OpParam::DerefI,
            "B" => OpParam::Digits, 
            "F" => OpParam::Fontset,
            regarg if regarg.starts_with("V") => {
                match u8::from_str_radix(&regarg[1..1], 16) {
                    Ok(regvl) => OpParam::Register(regvl),
                    _ => OpParam::Label(regarg.to_owned())
                }
            },
            constarg if constarg.starts_with("0x") => {
                match u16::from_str_radix(&constarg[2..], 16) {
                    Ok(constvl) => OpParam::Variable(constvl), 
                    _ => OpParam::Label(constarg.to_owned())
                }
            }
            "" => OpParam::Blank,
            other => OpParam::Label(other.to_owned()),
        }
    }
}

trait Instruction : Sized {
    
    fn to_opcode(&self) -> u16;

    fn parse_args(ln : &str) -> Result<Self, ParseError>;
}

trait InstructionWithLabels : Instruction {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Self;
}

macro_rules! parse_arg {
    ($instr_itr:ident) => {{
        let raw_part = $instr_itr.next();
        let parse = match raw_part {
            Some(rd) => OpParam::parse(rd),
            None => OpParam::Blank
        };
        parse
    }};
}

macro_rules! parse_args {
    ($ln:ident, 1) => {{
        let instruction_itr = $ln.splitn(2, ",");
        parse_arg!(instruction_itr)
    }};
    ($ln:ident, 2) => {{
        let instruction_itr = $ln.splitn(2, ",");

        let parsed_arg1 = parse_arg!(instruction_itr); 
        let parsed_arg2 = parse_arg!(instruction_itr);

        (parsed_arg1, parsed_arg2)
    }};
    ($ln:ident, 3) => {{
        let instruction_itr = $ln.splitn(2, ",");

        let parsed_arg1 = parse_arg!(instruction_itr); 
        let parsed_arg2 = parse_arg!(instruction_itr);
        let parse_length = parse_arg!(instruction_itr);

        (parsed_arg1, parsed_arg2, parse_length)
    }};
}

pub struct Load { 
    dest : OpParam,
    source :OpParam
}

impl Instruction for Load {

    fn to_opcode(&self) -> u16 {
        match (self.dest, self.source) {
            (OpParam::Register(regnum), OpParam::Variable(vnum)) => 0x6000 | ((regnum as u16 & 0x0F) << 8) | (vnum & 0x00FF),
            (OpParam::Register(dreg), OpParam::Register(sreg)) => 0x8000 | ((dreg as u16 & 0x0F) << 8) | ((sreg as u16 &0x0F << 4)),
            (OpParam::RegisterI, OpParam::Variable(vnum)) => 0xA000 | (vnum & 0x0FFF),
            (OpParam::Register(dreg), OpParam::Timer) => 0xF007 | ((dreg as u16) & 0x0F) << 8, 
            (OpParam::Register(dreg), OpParam::Keyboard) => 0xF00A | ((dreg as u16) &0x0F) << 8, 
            (OpParam::Timer, OpParam::Register(sreg)) => 0xF015 | ((sreg as u16 & 0x0F) << 8),
            (OpParam::AudioTimer, OpParam::Register(sreg)) => 0xF018 | ((sreg as u16 & 0x0F) << 8),
            (OpParam::Fontset, OpParam::Register(sreg)) => 0xF029 | ((sreg as u16 & 0x0F) << 8),
            (OpParam::Digits, OpParam::Register(sreg)) => 0xF033 | ((sreg as u16 & 0x0F) << 8),
            (OpParam::DerefI, OpParam::Register(sreg)) => 0xF055 | ((sreg as u16 & 0x0F) << 8),
            (OpParam::Register(dreg), OpParam::DerefI) => 0xF065 | ((dreg as u16 & 0x0F) << 8),
            _ => panic!("Could not correctly parse Load.")
        }
    }

    fn parse_args(ln: &str) -> Result<Load, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) | 
            (OpParam::Register(_), OpParam::Variable(_)) |
            (OpParam::Register(_), OpParam::Timer)       |
            (OpParam::Register(_), OpParam::DerefI)      |
            (OpParam::Register(_), OpParam::Keyboard)    |
            (OpParam::RegisterI, OpParam::Variable(_))   |
            (OpParam::RegisterI, OpParam::Label(_))   |
            (OpParam::Timer, OpParam::Register(_))       |
            (OpParam::AudioTimer, OpParam::Register(_))  |
            (OpParam::Fontset, OpParam::Register(_))     |
            (OpParam::Digits, OpParam::Register(_))      |
            (OpParam::DerefI, OpParam::Register(_))      => { Ok(Load{dest : parsed_dest, source : parsed_source}) },
            _ => Err(ParseError(format!("Could not parse load args: {}", ln)))
        }
    }
}

impl InstructionWithLabels for Load {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Load {
        let nlabel = labels.get(&self.source).unwrap_or(&self.source);
        Load{dest : self.dest.clone(), source : nlabel.clone()}
    }
}


pub struct ClearScreen {}

impl Instruction for ClearScreen {
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


pub struct Return {}

impl Instruction for Return {
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

pub struct Jump (OpParam, OpParam);

impl Instruction for Jump {
    fn to_opcode(&self) -> u16 {
        match *self {
            Jump(OpParam::Variable(addr), OpParam::Blank) => 0x1000 | (addr & 0x0FFF),
            Jump(OpParam::Label(lbl), OpParam::Blank) => panic!("Label {} not correclty replaced!", lbl),
            Jump(OpParam::Register(0), OpParam::Variable(addr)) => 0xB000 | (addr & 0x0FFF),
            _ => panic!("Could not correclty parse Jump.")
        }
    }

    fn parse_args(ln: &str) -> Result<Jump, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(0), OpParam::Label(_))    |
            (OpParam::Register(0), OpParam::Variable(_)) |
            (OpParam::Label(_), OpParam::Blank)          | 
            (OpParam::Variable(_), OpParam::Blank) => Ok(Jump(parsed_dest, parsed_source)),
            _ => Err(ParseError("Got invalid Jump dest!".to_owned()))
        }
    }
}

impl InstructionWithLabels for Jump {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Jump {
        let newLeft = labels.get(&self.0).unwrap_or(&self.0);
        let newRight = labels.get(&self.1).unwrap_or(&self.1);
        Jump(newLeft.clone(), newRight.clone())
    }
}

pub struct Call (OpParam);

impl Instruction for Call {
    fn to_opcode(&self) -> u16 {
        match *self {
            Call(OpParam::Variable(addr)) => 0x2000 | (addr & 0x0FFF),
            Call(OpParam::Label(lbl)) => panic!("Label {} not correclty replaced!", lbl),
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

impl InstructionWithLabels for Call {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Call {
        let newLeft = labels.get(&self.0).unwrap_or(&self.0);
        Call(newLeft.clone())
    }
}

pub struct SkipIfEqual(OpParam, OpParam);

impl Instruction for SkipIfEqual {
    fn to_opcode(&self) -> u16 {
        match *self {
            SkipIfEqual(OpParam::Register(dreg), OpParam::Variable(vl)) => 0x3000 | (dreg as u16 & 0xF) << 8 | (vl & 0xFF),
            SkipIfEqual(OpParam::Register(dreg), OpParam::Register(sreg)) => 0x5000 | (dreg as u16 & 0xF) << 8  | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid SkipIfEqual parameter!")
        }
    }

    
    fn parse_args(ln: &str) -> Result<SkipIfEqual, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) | 
            (OpParam::Register(_), OpParam::Variable(_)) => { Ok(SkipIfEqual(parsed_dest, parsed_source)) },
            _ => Err(ParseError(format!("Could not parse load args: {}", ln)))
        }
    }
}

pub struct SkipIfNotEqual(OpParam, OpParam);

impl Instruction for SkipIfNotEqual {
    fn to_opcode(&self) -> u16 {
        match *self {
            SkipIfNotEqual(OpParam::Register(dreg), OpParam::Variable(vl)) => 0x5000 | (dreg as u16 & 0xF) << 8 | (vl & 0xFF),
            SkipIfNotEqual(OpParam::Register(dreg), OpParam::Register(sreg)) => 0x9000 | (dreg as u16 & 0xF) << 8  | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid SkipIfEqual parameter!")
        }
    }

    
    fn parse_args(ln: &str) -> Result<SkipIfNotEqual, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) | 
            (OpParam::Register(_), OpParam::Variable(_)) => { Ok(SkipIfNotEqual(parsed_dest, parsed_source)) },
            _ => Err(ParseError(format!("Could not parse load args: {}", ln)))
        }
    }
}

pub struct Add {
    acc : OpParam, 
    toAdd : OpParam
}

impl Instruction for Add {
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

        match (parsed_dest, parsed_source) {
            (OpParam::RegisterI, OpParam::Register(_))   |
            (OpParam::Register(_), OpParam::Register(_)) | 
            (OpParam::Register(_), OpParam::Variable(_)) => { Ok(Add{acc: parsed_dest, toAdd: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse load args: {}", ln)))
        }
    }
}

pub struct Or {acc : OpParam, reg : OpParam}

impl Instruction for Or {
    fn to_opcode(&self) -> u16 {
        match *self {
            Or{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8001 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid Or parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<Or, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) => { Ok(Or{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse Or args: {}", ln)))
        }
    }
}

pub struct And {acc : OpParam, reg : OpParam}

impl Instruction for And {
    fn to_opcode(&self) -> u16 {
        match *self {
            And{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8002 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid And parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<And, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) => { Ok(And{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse And args: {}", ln)))
        }
    }
}

pub struct Xor {acc : OpParam, reg : OpParam}

impl Instruction for Xor {
    fn to_opcode(&self) -> u16 {
        match *self {
            Xor{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8003 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid Xor parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<Xor, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) => { Ok(Xor{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse Xor args: {}", ln)))
        }
    }
}

pub struct Sub {acc : OpParam, reg : OpParam}

impl Instruction for Sub {
    fn to_opcode(&self) -> u16 {
        match *self {
            Sub{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8005 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid Sub parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<Sub, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) => { Ok(Sub{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse Sub args: {}", ln)))
        }
    }
}

pub struct ShiftRight {
    acc : OpParam, 
    usually_unused : OpParam
}

impl Instruction for ShiftRight {
    fn parse_args(ln : &str) -> Result<ShiftRight, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) => { Ok(ShiftRight{acc: parsed_dest, usually_unused: parsed_source}) },
            (OpParam::Register(_), OpParam::Blank) => { Ok(ShiftRight{acc: parsed_dest, usually_unused: parsed_dest}) },
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

pub struct SubN {acc : OpParam, reg : OpParam}

impl Instruction for SubN {
    fn to_opcode(&self) -> u16 {
        match *self {
            SubN{acc : OpParam::Register(dreg), reg : OpParam::Register(sreg)} => 0x8007 | (dreg as u16 & 0xF) << 8 | (sreg as u16 & 0xF) << 4,
            _ => panic!("Got invalid SubN parameter!") 
        }
    }

    fn parse_args(ln : &str) -> Result<SubN, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) => { Ok(SubN{acc: parsed_dest, reg: parsed_source}) },
            _ => Err(ParseError(format!("Could not parse SubN args: {}", ln)))
        }
    }
}

pub struct ShiftLeft {
    acc : OpParam, 
    usually_unused : OpParam
}

impl Instruction for ShiftLeft {
    fn parse_args(ln : &str) -> Result<ShiftLeft, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Register(_)) => { Ok(ShiftLeft{acc: parsed_dest, usually_unused: parsed_source}) },
            (OpParam::Register(_), OpParam::Blank) => { Ok(ShiftLeft{acc: parsed_dest, usually_unused: parsed_dest}) },
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

pub struct Rand {
    reg : OpParam,
    mask : OpParam
}

impl Instruction for Rand {
    fn parse_args(ln : &str) -> Result<Rand, ParseError> {
        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (parsed_dest, parsed_source) {
            (OpParam::Register(_), OpParam::Variable(_)) => { Ok(Rand{reg: parsed_dest, mask: parsed_source}) },
            (OpParam::Register(_), OpParam::Blank) => { Ok(Rand{reg: parsed_dest, mask: OpParam::Variable(0x00FF)}) },
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

pub struct Draw {
    xreg : OpParam,
    yreg : OpParam,
    length : OpParam
}

impl Instruction for Draw {
    fn parse_args(ln : &str) -> Result<Draw, ParseError> {
        let (parsed_dest, parsed_source, parsed_len) = parse_args!(ln, 3);

        match (parsed_dest, parsed_source, parsed_len) {
            (OpParam::Register(_), OpParam::Register(_), OpParam::Variable(_)) => { Ok(Draw{xreg: parsed_dest, yreg: parsed_source, length : parsed_len}) },
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

pub struct SkipIfKey (OpParam);

impl Instruction for SkipIfKey {
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

pub struct SkipIfNotKey (OpParam);

impl Instruction for SkipIfNotKey {
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