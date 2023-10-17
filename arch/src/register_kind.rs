#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterKind {
    Regular { id: usize },
    ProgramCounter,
    StackPointer,
}
