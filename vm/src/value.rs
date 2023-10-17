#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Float { value: f32 },
    String { value: String },
}

impl Default for Value {
    fn default() -> Self {
        Self::Float { value: 0.0 }
    }
}
