#[derive(Debug)]
pub enum InterpreterError {
    NotImplemented,
    TypeMismatch,
    OperationUnsupported,
    UnknownVariable
}