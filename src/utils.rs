use crate::compiler_error::CompilerError;

pub type CompilerResult<T> = Result<T, CompilerError>;

pub trait Resettable {
    fn reset(&mut self);
}
