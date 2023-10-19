mod runtime_error;
mod value;
mod vm;
mod vm_status;

pub use runtime_error::RuntimeError;
pub use value::Value;
pub use vm::VM;
pub use vm_status::VMStatus;

pub const STACK_SIZE: usize = 16;
