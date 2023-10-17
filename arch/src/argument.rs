#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Register { id: usize },
    Pin { id: usize },
    Int { value: i32 },
    Float { value: f32 },
    String { value: String },
    Label { name: String },
}
