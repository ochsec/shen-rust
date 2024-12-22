//! Abstract Syntax Tree for Shen language constructs

#[derive(Debug, Clone)]
#[derive(Debug, Clone, PartialEq)]
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
            ShenNode::Literal { value } => match value {
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
            ShenNode::BinaryOperation { result_type, .. } => result_type.clone(),
            ShenNode::Conditional { true_branch, .. } => true_branch.get_type(),
            _ => ShenType::Symbol, // Default fallback
        }
    }

    pub fn try_convert(&self, target_type: &ShenType) -> Option<ShenNode> {
        match (self.get_type(), target_type) {
            // Numeric conversions
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
            // String conversions
            (ShenType::Integer, ShenType::String) => {
                if let ShenNode::Literal(ShenValue::Integer(val)) = self {
                    Some(ShenNode::Literal(ShenValue::String(val.to_string())))
                } else {
                    None
                }
            },
            (ShenType::Float, ShenType::String) => {
                if let ShenNode::Literal(ShenValue::Float(val)) = self {
                    Some(ShenNode::Literal(ShenValue::String(val.to_string())))
                } else {
                    None
                }
            },
            // Boolean conversions
            (ShenType::Integer, ShenType::Boolean) => {
                if let ShenNode::Literal(ShenValue::Integer(val)) = self {
                    Some(ShenNode::Literal(ShenValue::Boolean(*val != 0)))
                } else {
                    None
                }
            },
            (ShenType::Float, ShenType::Boolean) => {
                if let ShenNode::Literal(ShenValue::Float(val)) = self {
                    Some(ShenNode::Literal(ShenValue::Boolean(*val != 0.0)))
                } else {
                    None
                }
            },
            // Exact type match
            (from, to) if from == *to => Some(self.clone()),
            
            // Fallback for complex type inference
            _ => self.infer_complex_conversion(target_type),
        }
    }

    fn infer_complex_conversion(&self, target_type: &ShenType) -> Option<ShenNode> {
        match (self, target_type) {
            // Handle list conversions
            (ShenNode::List { elements, .. }, ShenType::List) => Some(self.clone()),
            
            // Handle symbol to specific type conversions
            (ShenNode::Symbol { name, .. }, target) => {
                // Try parsing symbol name to target type
                match target {
                    ShenType::Integer => name.parse::<i64>()
                        .map(|val| ShenNode::Literal(ShenValue::Integer(val))).ok(),
                    ShenType::Float => name.parse::<f64>()
                        .map(|val| ShenNode::Literal(ShenValue::Float(val))).ok(),
                    ShenType::Boolean => match name.to_lowercase().as_str() {
                        "true" => Some(ShenNode::Literal(ShenValue::Boolean(true))),
                        "false" => Some(ShenNode::Literal(ShenValue::Boolean(false))),
                        _ => None
                    },
                    _ => None
                }
            },
            
            // Default: no conversion possible
            _ => None
        }
    }

    /// Attempt to find the most appropriate type for a node
    pub fn infer_type(&self) -> ShenType {
        match self {
            ShenNode::Literal { value } => match value {
                ShenValue::Integer(_) => ShenType::Integer,
                ShenValue::Float(_) => ShenType::Float,
                ShenValue::String(_) => ShenType::String,
                ShenValue::Boolean(_) => ShenType::Boolean,
                ShenValue::Nil => ShenType::Nil,
            },
            ShenNode::Symbol { type_hint, name } => {
                // Enhanced type inference for symbols
                if let Ok(_) = name.parse::<i64>() {
                    ShenType::Integer
                } else if let Ok(_) = name.parse::<f64>() {
                    ShenType::Float
                } else if name.to_lowercase() == "true" || name.to_lowercase() == "false" {
                    ShenType::Boolean
                } else {
                    type_hint.clone()
                }
            },
            ShenNode::List { element_type, .. } => ShenType::List,
            ShenNode::Function { return_type, .. } => return_type.clone(),
            ShenNode::Lambda { return_type, .. } => return_type.clone(),
            ShenNode::BinaryOperation { result_type, .. } => result_type.clone(),
            ShenNode::Conditional { true_branch, .. } => true_branch.get_type(),
            ShenNode::Nil => ShenType::Nil,
            _ => ShenType::Symbol,
        }
    }
}
