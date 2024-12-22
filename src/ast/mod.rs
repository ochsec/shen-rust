//! Abstract Syntax Tree for Shen language constructs

#[derive(Debug, Clone)]
pub enum ShenType {
    Integer,
    Float,
    String,
    Boolean,
    Symbol,
    List,
    Function,
    Nil,
}

#[derive(Debug, Clone)]
pub enum ShenNode {
    Function {
        name: String,
        args: Vec<(String, ShenType)>,
        return_type: ShenType,
        body: Box<ShenNode>,
    },
    Application {
        func: Box<ShenNode>,
        args: Vec<ShenNode>,
    },
    Literal {
        value: ShenValue,
    },
    Symbol {
        name: String,
        type_hint: ShenType,
    },
    List {
        elements: Vec<ShenNode>,
        element_type: ShenType,
    },
    Lambda {
        args: Vec<(String, ShenType)>,
        return_type: ShenType,
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
        result_type: ShenType,
    },
    Nil,
}

#[derive(Debug, Clone)]
pub enum ShenValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl ShenNode {
    pub fn get_type(&self) -> ShenType {
        match self {
            ShenNode::Literal(value) => match value {
                ShenValue::Integer(_) => ShenType::Integer,
                ShenValue::Float(_) => ShenType::Float,
                ShenValue::String(_) => ShenType::String,
                ShenValue::Boolean(_) => ShenType::Boolean,
                ShenValue::Nil => ShenType::Nil,
            },
            ShenNode::Symbol { type_hint, .. } => type_hint.clone(),
            ShenNode::List { element_type, .. } => ShenType::List,
            ShenNode::Function { return_type, .. } => return_type.clone(),
            ShenNode::Lambda { return_type, .. } => return_type.clone(),
            ShenNode::Nil => ShenType::Nil,
            _ => ShenType::Symbol, // Default fallback
        }
    }

    pub fn try_convert(&self, target_type: &ShenType) -> Option<ShenNode> {
        match (self.get_type(), target_type) {
            (ShenType::Integer, ShenType::Float) => {
                if let ShenNode::Literal(ShenValue::Integer(val)) = self {
                    Some(ShenNode::Literal(ShenValue::Float(*val as f64)))
                } else {
                    None
                }
            },
            (ShenType::Float, ShenType::Integer) => {
                if let ShenNode::Literal(ShenValue::Float(val)) = self {
                    Some(ShenNode::Literal(ShenValue::Integer(*val as i64)))
                } else {
                    None
                }
            },
            (from, to) if from == *to => Some(self.clone()),
            _ => None,
        }
    }
}
