//! Code generation from AST to Rust

use quote::quote;
use syn;

use crate::ast::ShenNode;
use crate::error::TranspilerError;

pub fn generate_rust_code(node: &ShenNode) -> Result<String, TranspilerError> {
    match node {
        ShenNode::Function { name, args, body } => {
            let args_str = args.join(", ");
            let body_str = generate_rust_code(body)?;
            Ok(format!("fn {}({}) {{ {} }}", name, args_str, body_str))
        },
        ShenNode::Symbol { name } => Ok(name.clone()),
        ShenNode::Application { func, args } => {
            let func_str = generate_rust_code(func)?;
            let args_str = args.iter()
                .map(|arg| generate_rust_code(arg).unwrap_or_default())
                .collect::<Vec<_>>()
                .join(" ");
            Ok(format!("{} {}", func_str, args_str))
        },
        ShenNode::Conditional { condition, true_branch, false_branch } => {
            let condition_str = generate_rust_code(condition)?;
            let true_str = generate_rust_code(true_branch)?;
            let false_str = false_branch
                .as_ref()
                .map(|branch| generate_rust_code(branch).unwrap_or_default())
                .unwrap_or_else(|| "".to_string());
            
            Ok(format!("if ({}) {{ {} }}{}", 
                condition_str, 
                true_str, 
                if !false_str.is_empty() { format!(" else {{ {} }}", false_str) } else { "".to_string() }
            ))
        },
        ShenNode::Lambda { args, body } => {
            let args_str = args.join(", ");
            let body_str = generate_rust_code(body)?;
            Ok(format!("|{}| {{ {} }}", args_str, body_str))
        },
        _ => Err(TranspilerError::CodegenError("Unsupported node type".to_string())),
    }
}
