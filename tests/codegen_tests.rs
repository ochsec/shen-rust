//! Tests for Shen to Rust code generation

use shen_transpiler::codegen::generate_rust_code;
use shen_transpiler::parser::parse_shen_source;
use pretty_assertions::assert_eq;

// Add a helper function to simplify test code
fn assert_rust_code_generation(input: &str, expected_code: &str) {
    let parsed_node = parse_shen_source(input).expect("Parsing should succeed");
    let rust_code = generate_rust_code(&parsed_node);
    
    assert!(rust_code.is_ok(), "Code generation should succeed");
    assert_eq!(rust_code.unwrap(), expected_code);
}

#[test]
fn test_generate_simple_function() {
    let input = "(defun identity (x) x)";
    let parsed_node = parse_shen_source(input).expect("Parsing should succeed");
    
    let rust_code = generate_rust_code(&parsed_node);
    
    assert!(rust_code.is_ok(), "Code generation should succeed");
    
    let expected_code = "fn identity(x: impl Clone) -> impl Clone {\n    x\n}";
    assert_eq!(rust_code.unwrap(), expected_code);
}

#[test]
fn test_generate_function_with_multiple_args() {
    let input = "(defun add (x y) (+ x y))";
    let parsed_node = parse_shen_source(input).expect("Parsing should succeed");
    
    let rust_code = generate_rust_code(&parsed_node);
    
    assert!(rust_code.is_ok(), "Code generation should succeed");
    
    let expected_code = "fn add(x: impl Clone, y: impl Clone) -> impl Clone {\n    x + y\n}";
    assert_eq!(rust_code.unwrap(), expected_code);
}
