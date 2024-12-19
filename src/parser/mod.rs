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
    // Basic conditional parsing
    if tokens.len() < 5 {
        return Err("Invalid conditional".to_string());
    }

    let condition_tokens = &tokens[2..3];
    let true_branch_tokens = &tokens[3..4];
    let false_branch_tokens = if tokens.len() > 4 { &tokens[4..5] } else { &[] };

    let condition = parse_expression(condition_tokens)?;
    let true_branch = parse_expression(true_branch_tokens)?;
    let false_branch = if !false_branch_tokens.is_empty() {
        Some(Box::new(parse_expression(false_branch_tokens)?))
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
    // Placeholder for lambda parsing
    Err("Lambda parsing not implemented".to_string())
}

fn parse_list(tokens: &[Token]) -> Result<ShenNode, String> {
    // Placeholder for list parsing
    Err("List parsing not implemented".to_string())
}

fn parse_symbol_or_application(tokens: &[Token]) -> Result<ShenNode, String> {
    // Placeholder for symbol or application parsing
    Err("Symbol/Application parsing not implemented".to_string())
}
