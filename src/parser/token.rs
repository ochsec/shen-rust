#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    Defun,
    Lambda,
    Identifier(String),
    Literal(String),
    Symbol(String),
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
                    _ => Token::Identifier(identifier),
                });
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
