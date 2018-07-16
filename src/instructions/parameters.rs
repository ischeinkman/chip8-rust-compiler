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
                match u8::from_str_radix(&regarg[1..2], 16) {
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
            other => {
                OpParam::Label(other.to_owned())
            }
        }
    }
}