//! Abstract Syntax Tree for Shen language constructs

#[derive(Debug, Clone)]
pub enum ShenNode {
    Function {
        name: String,
        args: Vec<String>,
        body: Box<ShenNode>,
    },
    Application {
        func: Box<ShenNode>,
        args: Vec<ShenNode>,
    },
    Literal {
        value: String,
    },
    Symbol {
        name: String,
    },
    List {
        elements: Vec<ShenNode>,
    },
    Lambda {
        args: Vec<String>,
        body: Box<ShenNode>,
    },
    Conditional {
        condition: Box<ShenNode>,
        true_branch: Box<ShenNode>,
        false_branch: Option<Box<ShenNode>>,
    },
    BinaryOperation {
        operator: String,
        left: Box<ShenNode>,
        right: Box<ShenNode>,
    },
}
