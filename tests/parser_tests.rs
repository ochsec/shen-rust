//! Tests for Shen language parsing

use shen_transpiler::parser::parse_shen_source;
use shen_transpiler::ast::ShenNode;
use pretty_assertions::assert_eq;

#[test]
fn test_parse_simple_function() {
    let input = "(defun identity (x) x)";
    let result = parse_shen_source(input);
    
    assert!(result.is_ok(), "Parsing should succeed");
    
    let node = result.unwrap();
    match node {
        ShenNode::Function { name, args, body } => {
            assert_eq!(name, "identity");
            assert_eq!(args, vec!["x"]);
            // More detailed body checks can be added later
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
        ShenNode::Function { name, args, body } => {
            assert_eq!(name, "add");
            assert_eq!(args, vec!["x", "y"]);
        },
        _ => panic!("Expected a function node"),
    }
}
