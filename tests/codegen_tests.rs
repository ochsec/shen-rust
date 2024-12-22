//! Tests for Shen to Rust code generation

use shen_transpiler::codegen::generate_rust_code;
use shen_transpiler::parser::parse_shen_source;
use pretty_assertions::assert_eq;

// Add a helper function to simplify test code
fn assert_rust_code_generation(input: &str, expected_code: &str) {
    let parsed_node = parse_shen_source(input).expect("Parsing should succeed");
    let rust_code = generate_rust_code(&parsed_node);
    
    assert!(rust_code.is_ok(), "Code generation should succeed");
    assert_eq!(rust_code.unwrap().trim(), expected_code.trim());
}

#[test]
fn test_generate_simple_function() {
    let input = "(defun identity (x) x)";
    assert_rust_code_generation(
        input, 
        "fn identity(x: impl Clone) -> impl Clone {\n    x\n}"
    );
}

#[test]
fn test_generate_function_with_multiple_args() {
    let input = "(defun add (x y) (+ x y))";
    assert_rust_code_generation(
        input, 
        "fn add(x: impl Clone, y: impl Clone) -> impl Clone {\n    x + y\n}"
    );
}

#[test]
fn test_generate_lambda_expression() {
    let input = "(lambda (x) (+ x 1))";
    assert_rust_code_generation(
        input, 
        "|x| { x + 1 }"
    );
}

#[test]
fn test_generate_conditional() {
    let input = "(if (= x 0) x (+ x 1))";
    assert_rust_code_generation(
        input, 
        "if x == 0 {\n        x\n    } else {\n        x + 1\n    }"
    );
}

#[test]
fn test_generate_list_expression() {
    let input = "(list 1 2 3)";
    assert_rust_code_generation(
        input, 
        "vec![1.0, 2.0, 3.0]"
    );
}

#[test]
fn test_generate_binary_operation() {
    let input = "(+ 1 2)";
    assert_rust_code_generation(
        input, 
        "1.0 + 2.0"
    );
}
