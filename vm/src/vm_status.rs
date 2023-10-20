#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMStatus {
    Idle,
    Yield,
    Running,
    Finished,
    Error,
}
