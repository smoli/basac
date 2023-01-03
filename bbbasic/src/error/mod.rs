#[derive(Debug)]
pub enum InterpreterError {
    NotImplemented(String),
    TypeMismatch,
    OperationUnsupported,
    UnknownVariable(String),
    StatementNotFound,
    Unreachable
}