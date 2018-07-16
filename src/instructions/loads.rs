#[macro_use]
use instructions::*;

#[derive(Clone, Debug)]
pub struct Load { 
    dest : OpParam,
    source :OpParam
}

impl InstructionOps for Load {

    fn to_opcode(&self) -> u16 {
        match (&self.dest, &self.source) {
            (&OpParam::Register(regnum), &OpParam::Variable(vnum)) => 0x6000 | ((regnum as u16 & 0x0F) << 8) | (vnum & 0x00FF),
            (&OpParam::Register(dreg), &OpParam::Register(sreg)) => 0x8000 | ((dreg as u16 & 0x0F) << 8) | ((sreg as u16 &0x0F << 4)),
            (&OpParam::RegisterI, &OpParam::Variable(vnum)) => 0xA000 | (vnum & 0x0FFF),
            (&OpParam::Register(dreg), &OpParam::Timer) => 0xF007 | ((dreg as u16) & 0x0F) << 8, 
            (&OpParam::Register(dreg), &OpParam::Keyboard) => 0xF00A | ((dreg as u16) &0x0F) << 8, 
            (&OpParam::Timer, &OpParam::Register(sreg)) => 0xF015 | ((sreg as u16 & 0x0F) << 8),
            (&OpParam::AudioTimer, &OpParam::Register(sreg)) => 0xF018 | ((sreg as u16 & 0x0F) << 8),
            (&OpParam::Fontset, &OpParam::Register(sreg)) => 0xF029 | ((sreg as u16 & 0x0F) << 8),
            (&OpParam::Digits, &OpParam::Register(sreg)) => 0xF033 | ((sreg as u16 & 0x0F) << 8),
            (&OpParam::DerefI, &OpParam::Register(sreg)) => 0xF055 | ((sreg as u16 & 0x0F) << 8),
            (&OpParam::Register(dreg), &OpParam::DerefI) => 0xF065 | ((dreg as u16 & 0x0F) << 8),
            _ => panic!("Could not correctly parse Load.")
        }
    }

    fn parse_args(ln: &str) -> Result<Load, ParseError> {

        let (parsed_dest, parsed_source) = parse_args!(ln, 2);

        match (&parsed_dest, &parsed_source) {
            (&OpParam::Register(_), &OpParam::Register(_)) | 
            (&OpParam::Register(_), &OpParam::Variable(_)) |
            (&OpParam::Register(_), &OpParam::Timer)       |
            (&OpParam::Register(_), &OpParam::DerefI)      |
            (&OpParam::Register(_), &OpParam::Keyboard)    |
            (&OpParam::RegisterI, &OpParam::Variable(_))   |
            (&OpParam::RegisterI, &OpParam::Label(_))   |
            (&OpParam::Timer, &OpParam::Register(_))       |
            (&OpParam::AudioTimer, &OpParam::Register(_))  |
            (&OpParam::Fontset, &OpParam::Register(_))     |
            (&OpParam::Digits, &OpParam::Register(_))      |
            (&OpParam::DerefI, &OpParam::Register(_))      => { Ok(Load{dest : parsed_dest, source : parsed_source}) },
            _ => Err(ParseError(format!("Could not parse load args: {} => ({:?}. {:?})", ln, parsed_dest, parsed_source)))
        }
    }
}

impl InstructionOpsWithLabels for Load {
    fn resolve_labels(&self, labels : &HashMap<OpParam, OpParam>) -> Load {
        let nlabel = labels.get(&self.source).unwrap_or(&self.source);
        Load{dest : self.dest.clone(), source : nlabel.clone()}
    }
}