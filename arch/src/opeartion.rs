use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Sub,
    Mov,
    Mul,
    Div,
    Mod,
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
    Halt,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => f.write_str("add"),
            Operation::Sub => f.write_str("sub"),
            Operation::Mov => f.write_str("mov"),
            Operation::Mul => f.write_str("mul"),
            Operation::Div => f.write_str("div"),
            Operation::Mod => f.write_str("mod"),
            Operation::Dbg => f.write_str("dbg"),
            Operation::Yield => f.write_str("yield"),
            Operation::Beq => f.write_str("beq"),
            Operation::Beqz => f.write_str("beqz"),
            Operation::Bge => f.write_str("bge"),
            Operation::Bgez => f.write_str("bgez"),
            Operation::Bgt => f.write_str("bgt"),
            Operation::Bgtz => f.write_str("bgtz"),
            Operation::Ble => f.write_str("ble"),
            Operation::Blez => f.write_str("blez"),
            Operation::Blt => f.write_str("blt"),
            Operation::Bltz => f.write_str("bltz"),
            Operation::Bne => f.write_str("bne"),
            Operation::Bnez => f.write_str("bnez"),
            Operation::Seq => f.write_str("seq"),
            Operation::Seqz => f.write_str("seqz"),
            Operation::Sge => f.write_str("sge"),
            Operation::Sgez => f.write_str("sgez"),
            Operation::Sgt => f.write_str("sgt"),
            Operation::Sgtz => f.write_str("sgtz"),
            Operation::Sle => f.write_str("sle"),
            Operation::Slez => f.write_str("slez"),
            Operation::Slt => f.write_str("slt"),
            Operation::Sltz => f.write_str("sltz"),
            Operation::Sne => f.write_str("sne"),
            Operation::Snez => f.write_str("snez"),
            Operation::Halt => f.write_str("halt"),
        }
    }
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
            "halt" => Ok(Self::Halt),
            _ => Err(OpeartionParseError::UnknownOperation),
        }
    }
}
