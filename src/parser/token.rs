#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    Defun,
    Lambda,
    If,
    Let,
    List,  // Add List token
    Identifier(String),
    Literal(String),
    Symbol(String),
    Operator(String),
    Number(f64),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            },
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            },
            c if c.is_whitespace() => {
                chars.next();
            },
            '\\' | 'λ' => {
                tokens.push(Token::Lambda);
                chars.next();
            },
            '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                let mut op = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if "+-*/=<>".contains(next_ch) {
                        op.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Operator(op));
            },
            c if c.is_alphabetic() || c == '_' => {
                let mut identifier = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '?' || next_ch == '!' {
                        identifier.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                tokens.push(match identifier.as_str() {
                    "defun" => Token::Defun,
                    "lambda" | "fn" => Token::Lambda,
                    "if" => Token::If,
                    "let" => Token::Let,
                    "list" => Token::List,  // Add list token recognition
                    _ => Token::Identifier(identifier),
                });
            },
            c if c.is_numeric() => {
                let mut number = String::new();
                let mut is_float = false;
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_numeric() || next_ch == '.' {
                        if next_ch == '.' {
                            is_float = true;
                        }
                        number.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let parsed_number = if is_float {
                    number.parse::<f64>().map_err(|_| format!("Invalid number: {}", number))?
                } else {
                    number.parse::<f64>().map_err(|_| format!("Invalid number: {}", number))?
                };
                tokens.push(Token::Number(parsed_number));
            },
            '"' => {
                chars.next(); // consume opening quote
                let mut literal = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '"' {
                        chars.next(); // consume closing quote
                        break;
                    }
                    literal.push(next_ch);
                    chars.next();
                }
                tokens.push(Token::Literal(literal));
            },
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }

    Ok(tokens)
}
