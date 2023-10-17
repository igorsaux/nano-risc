use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Sub,
    Mov,
    Mul,
    Div,
    Mod,
    Jmp,
    Dbg,
    Yield,
    Beq,
    Beqz,
    Bge,
    Bgez,
    Bgt,
    Bgtz,
    Ble,
    Blez,
    Blt,
    Bltz,
    Bne,
    Bnez,
    Seq,
    Seqz,
    Sge,
    Sgez,
    Sgt,
    Sgtz,
    Sle,
    Slez,
    Slt,
    Sltz,
    Sne,
    Snez,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpeartionParseError {
    UnknownOperation,
}

impl FromStr for Operation {
    type Err = OpeartionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "mov" => Ok(Self::Mov),
            "mul" => Ok(Self::Mul),
            "div" => Ok(Self::Div),
            "mod" => Ok(Self::Mod),
            "jmp" => Ok(Self::Jmp),
            "dbg" => Ok(Self::Dbg),
            "yield" => Ok(Self::Yield),
            "beq" => Ok(Self::Beq),
            "beqz" => Ok(Self::Beqz),
            "bge" => Ok(Self::Bge),
            "bgez" => Ok(Self::Bgez),
            "bgt" => Ok(Self::Bgt),
            "bgtz" => Ok(Self::Bgtz),
            "ble" => Ok(Self::Ble),
            "blez" => Ok(Self::Blez),
            "blt" => Ok(Self::Blt),
            "bltz" => Ok(Self::Bltz),
            "bne" => Ok(Self::Bne),
            "bnez" => Ok(Self::Bnez),
            "seq" => Ok(Self::Seq),
            "seqz" => Ok(Self::Seqz),
            "sge" => Ok(Self::Sge),
            "sgez" => Ok(Self::Sgez),
            "sgt" => Ok(Self::Sgt),
            "sgtz" => Ok(Self::Sgtz),
            "sle" => Ok(Self::Sle),
            "slez" => Ok(Self::Slez),
            "slt" => Ok(Self::Slt),
            "sltz" => Ok(Self::Sltz),
            "sne" => Ok(Self::Sne),
            "snez" => Ok(Self::Snez),
            _ => Err(OpeartionParseError::UnknownOperation),
        }
    }
}
