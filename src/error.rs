//! Error handling for the Shen transpiler

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranspilerError {
    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Code generation error: {0}")]
    CodegenError(String),

    #[error("Unsupported language construct: {0}")]
    UnsupportedConstruct(String),
}
