use crate::RegisterKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Register { kind: RegisterKind },
    Pin { id: usize },
    Int { value: i32 },
    Float { value: f32 },
    String { value: String },
    Label { name: String },
}
