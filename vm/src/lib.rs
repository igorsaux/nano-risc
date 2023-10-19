mod compile_error;
mod limits;
mod program;
mod runtime_error;
mod value;
mod vm;
mod vm_status;

pub use compile_error::CompileError;
pub use limits::Limits;
pub use program::Program;
pub use runtime_error::RuntimeError;
pub use value::Value;
pub use vm::VM;
pub use vm_status::VMStatus;

pub const MAX_PINS: usize = 16;
pub const STACK_SIZE: usize = 16;
