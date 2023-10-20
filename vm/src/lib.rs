mod runtime_error;
mod runtime_error_kind;
mod value;
mod vm;
mod vm_status;

pub use runtime_error::RuntimeError;
pub use runtime_error_kind::RuntimeErrorKind;
pub use value::Value;
pub use vm::VM;
pub use vm_status::VMStatus;

pub const STACK_SIZE: usize = 16;
