#[derive(Debug)]
pub enum InterpreterError {
    TypeMismatch,
    OperationUnsupported,
    UnknownVariable
}