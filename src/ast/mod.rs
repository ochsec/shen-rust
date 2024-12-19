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
}
