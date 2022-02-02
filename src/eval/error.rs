#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    UndefinedBinding,
    CallingANonFunctionValue,
    WrongNumberOfArguments,
    TypeError,
    InternalError,
}
