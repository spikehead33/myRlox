use std::{error::Error, fmt::Debug};

#[derive(thiserror::Error)]
pub enum CompilerError {
    #[error("no such file")]
    NoSuchFileError(#[source] std::io::Error),
    #[error("will be filled in")]
    LexerError,
}

impl Debug for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}
