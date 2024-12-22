//! Code generation from AST to Rust

use quote::quote;
use syn;

use crate::ast::ShenNode;
use crate::error::TranspilerError;

pub fn generate_rust_code(node: &ShenNode) -> Result<String, TranspilerError> {
    match node {
        ShenNode::Function { name, args, body, return_type } => {
            let args_str = args.iter()
                .map(|(name, _type)| format!("{}: impl Clone", name))
                .collect::<Vec<_>>()
                .join(", ");
            let body_str = generate_rust_code(body)?;
            Ok(format!("fn {}({}) -> impl Clone {{\n    {}\n}}", name, args_str, body_str))
        },
        ShenNode::Symbol { name, type_hint } => Ok(name.clone()),
        ShenNode::Nil => Ok("None".to_string()),
        ShenNode::Application { func, args } => {
            let func_str = generate_rust_code(func)?;
            let args_str = args.iter()
                .map(|arg| generate_rust_code(arg).unwrap_or_default())
                .collect::<Vec<_>>()
                .join(", ");
            
            // Special handling for common Shen functions and special forms
            match func_str.as_str() {
                "length" => Ok(format!("{}.len()", args_str)),
                "first" => Ok(format!("{}.first()", args_str)),
                "if" => {
                    // Handle if as a ternary-like operation
                    if args.len() == 3 {
                        let condition = generate_rust_code(&args[0])?;
                        let true_branch = generate_rust_code(&args[1])?;
                        let false_branch = generate_rust_code(&args[2])?;
                        Ok(format!("if {} {{ {} }} else {{ {} }}", condition, true_branch, false_branch))
                    } else {
                        Err(TranspilerError::CodegenError("Invalid 'if' expression".to_string()))
                    }
                },
                _ => Ok(format!("{}({})", func_str, args_str)),
            }
        },
        ShenNode::BinaryOperation { operator, left, right, result_type } => {
            let left_str = generate_rust_code(left)?;
            let right_str = generate_rust_code(right)?;
            
            // Map Shen operators to Rust equivalents
            let rust_op = match operator.as_str() {
                "=" => "==",
                "<" => "<",
                ">" => ">",
                "-" => "-",
                "+" => "+",
                _ => operator,
            };
            
            Ok(format!("{} {} {}", left_str, rust_op, right_str))
        },
        ShenNode::Conditional { condition, true_branch, false_branch } => {
            let condition_str = generate_rust_code(condition)?;
            let true_str = generate_rust_code(true_branch)?;
            let false_str = false_branch
                .as_ref()
                .map(|branch| generate_rust_code(branch).unwrap_or_default())
                .unwrap_or_else(|| "None".to_string());
            
            Ok(format!("if {} {{\n        {}\n    }} else {{\n        {}\n    }}", 
                condition_str, 
                true_str, 
                false_str
            ))
        },
        ShenNode::Lambda { args, body, return_type } => {
            let args_str = args.iter()
                .map(|(name, _type)| name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            let body_str = generate_rust_code(body)?;
            Ok(format!("|{}| {{ {} }}", args_str, body_str))
        },
        ShenNode::List { elements, element_type } => {
            let elements_str = elements.iter()
                .map(|elem| generate_rust_code(elem).unwrap_or_default())
                .collect::<Vec<_>>()
                .join(", ");
            Ok(format!("vec![{}]", elements_str))
        },
        _ => Err(TranspilerError::CodegenError("Unsupported node type".to_string())),
    }
}
