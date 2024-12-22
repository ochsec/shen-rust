//! Tests for Shen language parsing

use pretty_assertions::assert_eq;
use shen_transpiler::ast::{ShenNode, ShenType, ShenValue};
use shen_transpiler::parser::parse_shen_source;

#[test]
fn test_parse_simple_function() {
    let input = "(defun identity (x) x)";
    let result = parse_shen_source(input);

    assert!(result.is_ok(), "Parsing should succeed");

    let node = result.unwrap();
    match node {
        ShenNode::Function {
            name,
            args,
            body,
            return_type,
            ..
        } => {
            assert_eq!(name, "identity");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].0, "x");
            assert_eq!(args[0].1, ShenType::Symbol);

            // Check body is a symbol
            match *body {
                ShenNode::Symbol { name, type_hint } => {
                    assert_eq!(name, "x");
                    assert_eq!(type_hint, ShenType::Symbol);
                }
                _ => panic!("Expected a symbol body"),
            }

            // Check return type inference
            assert_eq!(return_type, ShenType::Symbol);
        }
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
        ShenNode::Function {
            name,
            args,
            body,
            return_type,
            ..
        } => {
            assert_eq!(name, "add");
            assert_eq!(args.len(), 2);
            assert_eq!(args[0].0, "x");
            assert_eq!(args[1].0, "y");

            // Check body is a binary operation
            match *body {
                ShenNode::BinaryOperation {
                    operator,
                    left,
                    right,
                    result_type,
                    ..
                } => {
                    assert_eq!(operator, "+");
                    assert_eq!(result_type, ShenType::Float);

                    match (*left, *right) {
                        (
                            ShenNode::Symbol {
                                name: left_name,
                                type_hint: left_type,
                                ..
                            },
                            ShenNode::Symbol {
                                name: right_name,
                                type_hint: right_type,
                                ..
                            },
                        ) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(right_name, "y");
                            assert_eq!(left_type, ShenType::Symbol);
                            assert_eq!(right_type, ShenType::Symbol);
                        }
                        _ => panic!("Expected symbol operands"),
                    }
                }
                _ => panic!("Expected a binary operation"),
            }

            // Check return type inference
            assert_eq!(return_type, ShenType::Float);
        }
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
        ShenNode::Lambda {
            args,
            body,
            return_type,
            ..
        } => {
            assert_eq!(args.len(), 1);
            assert_eq!(args[0].0, "x");
            assert_eq!(args[0].1, ShenType::Symbol);

            // Check body is a binary operation
            match *body {
                ShenNode::BinaryOperation {
                    operator,
                    left,
                    right,
                    result_type,
                    ..
                } => {
                    assert_eq!(operator, "+");
                    assert_eq!(result_type, ShenType::Float);

                    match (*left, *right) {
                        (
                            ShenNode::Symbol {
                                name: left_name,
                                type_hint: left_type,
                                ..
                            },
                            ShenNode::Literal { value },
                        ) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(left_type, ShenType::Symbol);
                            assert_eq!(value, ShenValue::Float(1.0));
                        }
                        _ => panic!("Expected symbol and literal operands"),
                    }
                }
                _ => panic!("Expected a binary operation"),
            }

            // Check return type inference
            assert_eq!(return_type, ShenType::Float);
        }
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
        ShenNode::Conditional {
            condition,
            true_branch,
            false_branch,
        } => {
            // Check condition
            match *condition {
                ShenNode::BinaryOperation {
                    operator,
                    left,
                    right,
                    result_type,
                    ..
                } => {
                    assert_eq!(operator, "=");
                    assert_eq!(result_type, ShenType::Boolean);

                    match (*left, *right) {
                        (
                            ShenNode::Symbol {
                                name: left_name,
                                type_hint: left_type,
                                ..
                            },
                            ShenNode::Literal { value },
                        ) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(left_type, ShenType::Symbol);
                            // Compare the numeric value directly
                            match value {
                                ShenValue::Float(val) => assert_eq!(val, 0.0),
                                _ => panic!("Expected Float value"),
                            }
                        }
                        _ => panic!("Expected symbol and literal in condition"),
                    }
                }
                _ => panic!("Expected a binary operation as condition"),
            }

            // Check true branch
            match *true_branch {
                ShenNode::Symbol {
                    name, type_hint, ..
                } => {
                    assert_eq!(name, "x");
                    assert_eq!(type_hint, ShenType::Symbol);
                }
                _ => panic!("Expected a symbol in true branch"),
            }

            // Check false branch
            match *false_branch.unwrap() {
                ShenNode::BinaryOperation {
                    operator,
                    left,
                    right,
                    result_type,
                    ..
                } => {
                    assert_eq!(operator, "+");
                    assert_eq!(result_type, ShenType::Float);

                    match (*left, *right) {
                        (
                            ShenNode::Symbol {
                                name: left_name,
                                type_hint: left_type,
                                ..
                            },
                            ShenNode::Literal { value },
                        ) => {
                            assert_eq!(left_name, "x");
                            assert_eq!(left_type, ShenType::Symbol);
                            // Compare the numeric value directly
                            match value {
                                ShenValue::Float(val) => assert_eq!(val, 1.0),
                                _ => panic!("Expected Float value"),
                            }
                        }
                        _ => panic!("Expected symbol and literal in false branch"),
                    }
                }
                _ => panic!("Expected a binary operation in false branch"),
            }
        }
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
        ShenNode::List {
            elements,
            element_type,
        } => {
            assert_eq!(elements.len(), 3);
            assert_eq!(element_type, ShenType::Float);

            for (i, elem) in elements.iter().enumerate() {
                match elem {
                    ShenNode::Literal { value } => {
                        assert_eq!(*value, );
                    }
                    _ => panic!("Expected literal elements"),
                }
            }
        }
        _ => panic!("Expected a list node"),
    }
}
