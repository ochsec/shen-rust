//! Error handling for the Shen transpiler

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranspilerError {
    #[error("Parsing error at token {token}: {message}")]
    ParseError {
        token: String,
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Code generation error: {0}")]
    CodegenError(String),

    #[error("Unsupported language construct: {0}")]
    UnsupportedConstruct(String),

    #[error("Type conversion error: Cannot convert {from} to {to}")]
    TypeConversionError {
        from: String,
        to: String,
    },

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Internal transpiler error: {0}")]
    InternalError(String),
}

impl TranspilerError {
    pub fn new_parse_error(token: &str, message: &str, line: usize, column: usize) -> Self {
        TranspilerError::ParseError {
            token: token.to_string(),
            message: message.to_string(),
            line,
            column,
        }
    }

    pub fn new_type_conversion_error(from: &str, to: &str) -> Self {
        TranspilerError::TypeConversionError {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}
