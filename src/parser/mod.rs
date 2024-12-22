//! Parser for Shen language

mod token;

use crate::ast::ShenNode;
use crate::error::TranspilerError;
use token::{Token, tokenize};

pub fn parse_shen_source(input: &str) -> Result<ShenNode, TranspilerError> {
    let tokens = tokenize(input)
        .map_err(|e| TranspilerError::ParseError(e))?;
    
    parse_expression(&tokens)
        .map_err(|e| TranspilerError::ParseError(e.to_string()))
}

fn parse_expression(tokens: &[Token]) -> Result<ShenNode, String> {
    if tokens.is_empty() {
        return Err("Empty input".to_string());
    }

    match &tokens[0] {
        Token::OpenParen => parse_complex_expression(tokens),
        Token::Defun => parse_function_definition(tokens),
        Token::Lambda => parse_lambda(tokens),
        Token::Identifier(_) => parse_symbol_or_application(tokens),
        _ => Err("Unexpected token".to_string())
    }
}

fn parse_complex_expression(tokens: &[Token]) -> Result<ShenNode, String> {
    if tokens.len() < 2 {
        return Err("Invalid complex expression".to_string());
    }

    match &tokens[1] {
        Token::Identifier(op) if op == "if" => parse_conditional(tokens),
        Token::Identifier(_) => parse_application(tokens),
        _ => parse_list(tokens)
    }
}

fn parse_conditional(tokens: &[Token]) -> Result<ShenNode, String> {
    // More robust conditional parsing
    if tokens.len() < 4 {
        return Err("Invalid conditional".to_string());
    }

    // Find the end of the condition
    let mut condition_end = 2;
    let mut paren_count = 0;
    for (i, token) in tokens[2..].iter().enumerate() {
        match token {
            Token::OpenParen => paren_count += 1,
            Token::CloseParen => {
                if paren_count == 0 {
                    condition_end = i + 2;
                    break;
                }
                paren_count -= 1;
            },
            _ => {}
        }
    }

    let condition_tokens = &tokens[2..condition_end];
    let true_branch_tokens = &tokens[condition_end..];
    
    let condition = parse_expression(condition_tokens)?;
    
    // Handle multiple possible true branch scenarios
    let true_branch = if !true_branch_tokens.is_empty() {
        parse_expression(true_branch_tokens)?
    } else {
        return Err("Missing true branch in conditional".to_string());
    };

    // Check for false branch
    let false_branch = if true_branch_tokens.len() > 1 {
        Some(Box::new(parse_expression(&true_branch_tokens[1..])?))
    } else {
        None
    };

    Ok(ShenNode::Conditional {
        condition: Box::new(condition),
        true_branch: Box::new(true_branch),
        false_branch,
    })
}

fn parse_application(tokens: &[Token]) -> Result<ShenNode, String> {
    // Basic application parsing
    if tokens.len() < 2 {
        return Err("Invalid application".to_string());
    }

    let func_token = &tokens[1];
    let args_tokens = &tokens[2..];

    let func = match func_token {
        Token::Identifier(name) => ShenNode::Symbol { name: name.clone() },
        _ => return Err("Invalid function in application".to_string()),
    };

    let mut args = Vec::new();
    for arg_token in args_tokens {
        match arg_token {
            Token::Identifier(name) => args.push(ShenNode::Symbol { name: name.clone() }),
            _ => return Err("Invalid argument in application".to_string()),
        }
    }

    Ok(ShenNode::Application {
        func: Box::new(func),
        args,
    })
}

fn parse_function_definition(tokens: &[Token]) -> Result<ShenNode, String> {
    // Basic implementation, needs more robust parsing
    if tokens.len() < 4 {
        return Err("Invalid function definition".to_string());
    }

    match &tokens[1..] {
        [Token::Identifier(name), Token::OpenParen, rest @ ..] => {
            let mut args = Vec::new();
            let mut body_tokens = Vec::new();
            let mut in_args = true;

            for token in rest {
                match token {
                    Token::CloseParen => {
                        in_args = false;
                        continue;
                    },
                    Token::Identifier(arg) if in_args => {
                        args.push(arg.clone());
                    },
                    _ if !in_args => {
                        body_tokens.push(token.clone());
                    },
                    _ => return Err("Invalid function definition".to_string()),
                }
            }

            let body = parse_expression(&body_tokens)?;

            Ok(ShenNode::Function {
                name: name.clone(),
                args,
                body: Box::new(body),
            })
        },
        _ => Err("Invalid function definition".to_string()),
    }
}

fn parse_lambda(tokens: &[Token]) -> Result<ShenNode, String> {
    // Lambda syntax: (lambda (arg1 arg2 ...) body)
    if tokens.len() < 4 {
        return Err("Invalid lambda expression".to_string());
    }

    // Find the arguments list
    let args_start = match tokens.get(1) {
        Some(Token::OpenParen) => 2,
        _ => return Err("Lambda arguments must be enclosed in parentheses".to_string()),
    };

    // Collect lambda arguments
    let mut args = Vec::new();
    let mut body_start = args_start;
    for (i, token) in tokens[args_start..].iter().enumerate() {
        match token {
            Token::Identifier(arg) => {
                args.push(arg.clone());
            },
            Token::CloseParen => {
                body_start = args_start + i + 1;
                break;
            },
            _ => return Err("Invalid lambda argument".to_string()),
        }
    }

    // Parse lambda body
    let body_tokens = &tokens[body_start..];
    let body = parse_expression(body_tokens)?;

    Ok(ShenNode::Lambda {
        args,
        body: Box::new(body),
    })
}

fn parse_list(tokens: &[Token]) -> Result<ShenNode, String> {
    // Placeholder for list parsing
    Err("List parsing not implemented".to_string())
}

fn parse_symbol_or_application(tokens: &[Token]) -> Result<ShenNode, String> {
    // Placeholder for symbol or application parsing
    Err("Symbol/Application parsing not implemented".to_string())
}
