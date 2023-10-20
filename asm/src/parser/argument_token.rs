use nano_risc_arch::RegisterKind;

#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentToken {
    Register { register: RegisterKind },
    Pin { id: usize },
    Int { value: i32 },
    Float { value: f32 },
    String { value: String },
    Label { name: String },
}