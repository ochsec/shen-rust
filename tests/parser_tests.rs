//! Tests for Shen language parsing

use shen_transpiler::parser::parse_shen_source;
use shen_transpiler::ast::{ShenNode, ShenType, ShenValue};
use pretty_assertions::assert_eq;

#[test]
fn test_parse_simple_function() {
    let input = "(defun identity (x) x)";
    let result = parse_shen_source(input);
    
    assert!(result.is_ok(), "Parsing should succeed");
    
    let node = result.unwrap();
    match node {
        ShenNode::Function { name, args, body, .. } => {
            assert_eq!(name, "identity");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].0, "x");
            assert_eq!(args[0].1, ShenType::Symbol);
            
            // Check body is a symbol
            match *body {
                ShenNode::Symbol { name, type_hint } => {
                    assert_eq!(name, "x");
                    assert_eq!(type_hint, ShenType::Symbol);
                },
                _ => panic!("Expected a symbol body"),
            }
        },
        _ => panic!("Expected a function node"),
    }
}

#[test]
fn test_parse_function_with_multiple_args() {
    let input = "(defun add (x y) (+ x y))";
    let result = parse_shen_source(input);
    
    assert!(result.is_ok(), "Parsing should succeed");
    
    let node = result.unwrap();
    match node {
        ShenNode::Function { name, args, body, .. } => {
            assert_eq!(name, "add");
            assert_eq!(args.len(), 2);
            assert_eq!(args[0].0, "x");
            assert_eq!(args[1].0, "y");
            
            // Check body is a binary operation
            match *body {
                ShenNode::BinaryOperation { operator, left, right, .. } => {
                    assert_eq!(operator, "+");
                    
                    match (*left, *right) {
                        (ShenNode::Symbol { name: left_name, .. }, 
                         ShenNode::Symbol { name: right_name, .. }) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(right_name, "y");
                        },
                        _ => panic!("Expected symbol operands"),
                    }
                },
                _ => panic!("Expected a binary operation"),
            }
        },
        _ => panic!("Expected a function node"),
    }
}

#[test]
fn test_parse_lambda_expression() {
    let input = "(lambda (x) (+ x 1))";
    let result = parse_shen_source(input);
    
    assert!(result.is_ok(), "Parsing should succeed");
    
    let node = result.unwrap();
    match node {
        ShenNode::Lambda { args, body, .. } => {
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].0, "x");
            
            // Check body is a binary operation
            match *body {
                ShenNode::BinaryOperation { operator, left, right, .. } => {
                    assert_eq!(operator, "+");
                    
                    match (*left, *right) {
                        (ShenNode::Symbol { name: left_name, .. }, 
                         ShenNode::Literal { value } ) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(value, ShenValue::Float(1.0));
                        },
                        _ => panic!("Expected symbol and literal operands"),
                    }
                },
                _ => panic!("Expected a binary operation"),
            }
        },
        _ => panic!("Expected a lambda node"),
    }
}

#[test]
fn test_parse_conditional() {
    let input = "(if (= x 0) x (+ x 1))";
    let result = parse_shen_source(input);
    
    assert!(result.is_ok(), "Parsing should succeed");
    
    let node = result.unwrap();
    match node {
        ShenNode::Conditional { condition, true_branch, false_branch } => {
            // Check condition
            match *condition {
                ShenNode::BinaryOperation { operator, left, right, .. } => {
                    assert_eq!(operator, "=");
                    
                    match (*left, *right) {
                        (ShenNode::Symbol { name: left_name, .. }, 
                         ShenNode::Literal { value } ) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(value, ShenValue::Float(0.0));
                        },
                        _ => panic!("Expected symbol and literal in condition"),
                    }
                },
                _ => panic!("Expected a binary operation as condition"),
            }
            
            // Check true branch
            match *true_branch {
                ShenNode::Symbol { name, .. } => {
                    assert_eq!(name, "x");
                },
                _ => panic!("Expected a symbol in true branch"),
            }
            
            // Check false branch
            match *false_branch.unwrap() {
                ShenNode::BinaryOperation { operator, left, right, .. } => {
                    assert_eq!(operator, "+");
                    
                    match (*left, *right) {
                        (ShenNode::Symbol { name: left_name, .. }, 
                         ShenNode::Literal { value } ) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(value, ShenValue::Float(1.0));
                        },
                        _ => panic!("Expected symbol and literal in false branch"),
                    }
                },
                _ => panic!("Expected a binary operation in false branch"),
            }
        },
        _ => panic!("Expected a conditional node"),
    }
}

#[test]
fn test_parse_list_expression() {
    let input = "(list 1 2 3)";
    let result = parse_shen_source(input);
    
    assert!(result.is_ok(), "Parsing should succeed");
    
    let node = result.unwrap();
    match node {
        ShenNode::List { elements, element_type } => {
            assert_eq!(elements.len(), 3);
            assert_eq!(element_type, ShenType::Float);
            
            for (i, elem) in elements.iter().enumerate() {
                match elem {
                    ShenNode::Literal { value } => {
                        assert_eq!(*value, ShenValue::Float((i + 1) as f64));
                    },
                    _ => panic!("Expected literal elements"),
                }
            }
        },
        _ => panic!("Expected a list node"),
    }
}
