pub fn tokenize_input(file_contents: Vec<char>) -> Vec<Token> {
    let mut current: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        if current >= file_contents.len() {
            break;
        }
        match file_contents.get(current).unwrap() {
            // Handle single character tokens
            ';' => {
                tokens.push(Token::SEMICOLON);
                current += 1;
            }
            ',' => {
                tokens.push(Token::COMMA);
                current += 1;
            }
            '+' => {
                tokens.push(Token::ADD);
                current += 1;
            }
            '-' => {
                tokens.push(Token::SUBTRACT);
                current += 1;
            }
            '^' => {
                tokens.push(Token::XOR);
                current += 1;
            }
            '(' => {
                tokens.push(Token::OPENPARENTHESES);
                current += 1;
            }
            ')' => {
                tokens.push(Token::CLOSEPARENTHESES);
                current += 1;
            }
            '{' => {
                tokens.push(Token::OPENBRACE);
                current += 1;
            }
            '}' => {
                tokens.push(Token::CLOSEBRACE);
                current += 1;
            }
            '[' => {
                tokens.push(Token::OPENBRACKET);
                current += 1;
            }
            ']' => {
                tokens.push(Token::CLOSEBRACKET);
                current += 1;
            }
            // Handle multi-character tokens
            '*' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '*' => tokens.push(Token::EXPONENT),
                    _ => tokens.push(Token::MULTIPLY),
                }
                tokens.push(Token::MULTIPLY);
                current += 1;
            }
            '.' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '.' => tokens.push(Token::DOUBLEDOT),
                    _ => tokens.push(Token::DOT),
                }
            }
            '=' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '=' => tokens.push(Token::DOUBLEEQUALS),
                    _ => tokens.push(Token::EQUALS),
                }
            }
            '/' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '/' => {
                        // Handle comment
                        while *file_contents.get(current).unwrap() != '\n' {
                            current += 1;
                        }
                        current += 1;
                    }
                    _ => tokens.push(Token::DIVIDE),
                }
            }
            '>' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '=' => tokens.push(Token::GREATEREQUALS),
                    '>' => tokens.push(Token::BITSRIGHT),
                    _ => tokens.push(Token::GREATERTHAN),
                }
            }
            '<' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '=' => tokens.push(Token::LESSEQUALS),
                    '<' => tokens.push(Token::BITSLEFT),
                    _ => tokens.push(Token::LESSTHAN),
                }
            }
            '!' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '=' => tokens.push(Token::NOTEQUALS),
                    _ => tokens.push(Token::NEGATION),
                }
            }
            '&' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '&' => tokens.push(Token::AND),
                    _ => tokens.push(Token::BITWISEAND),
                }
            }
            '|' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '|' => tokens.push(Token::OR),
                    _ => tokens.push(Token::BITWISEOR),
                }
            }
            '"' => {
                // Handle String datatype
                let mut buf = String::new();
                current += 1;
                while *file_contents.get(current).unwrap() != '"' {
                    buf.push(*file_contents.get(current).unwrap());
                    current += 1;
                }
                tokens.push(Token::STRINGLITERAL(buf));
                current += 1;
            }
            'a'..='z' => {
                // Handle identifiers starting with lowercase
                let mut buf: String = String::new();
                while file_contents.get(current).unwrap().is_alphanumeric() {
                    buf.push(*file_contents.get(current).unwrap());
                    current += 1;
                }
                match buf.as_str() {
                    // Handle keywords
                    "digit" => tokens.push(Token::DIGIT),
                    "float" => tokens.push(Token::FLOAT),
                    "string" => tokens.push(Token::STRING),
                    "let" => tokens.push(Token::LET),
                    "var" => tokens.push(Token::VAR),
                    "if" => tokens.push(Token::IF),
                    "else" => tokens.push(Token::ELSE),
                    "for" => tokens.push(Token::FOR),
                    "while" => tokens.push(Token::WHILE),
                    "break" => tokens.push(Token::BREAK),
                    "put" => tokens.push(Token::PUT),
                    "continue" => tokens.push(Token::CONTINUE),
                    "func" => tokens.push(Token::FUNC),
                    "in" => tokens.push(Token::IN),
                    _ => tokens.push(Token::IDENTIFIER(buf)),
                }
            }
            'A'..='Z' => {
                // Handle identifiers starting with uppercase
                let mut buf: String = String::new();
                while file_contents.get(current).unwrap().is_alphanumeric() {
                    buf.push(*file_contents.get(current).unwrap());
                    current += 1;
                }
                tokens.push(Token::IDENTIFIER(buf));
            }
            '0'..='9' => {
                let mut buf: String = String::new();
                let mut current_char: char = *file_contents.get(current).unwrap();
                while current_char.is_digit(10) || current_char == '.' {
                    buf.push(*file_contents.get(current).unwrap());
                    current += 1;
                    current_char = *file_contents.get(current).unwrap();
                }
                if let Ok(item) = buf.parse::<i128>() {
                    // Handle digit literals
                    tokens.push(Token::DIGITLITERAL(item));
                } else if let Ok(item) = buf.parse::<f64>() {
                    // Handle floating point literals
                    tokens.push(Token::FPLITERAL(item));
                } else {
                    tokens.push(Token::INVALID);
                }
            }
            _ => {
                if !file_contents.get(current).unwrap().is_whitespace() {
                    tokens.push(Token::INVALID)
                }
                current += 1;
            }
        }
    }
    tokens
}

#[derive(Debug)]
pub enum Token {
    IDENTIFIER(String),
    STRINGLITERAL(String),
    FPLITERAL(f64),
    DIGITLITERAL(i128),
    ADD,
    DOT,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    EXPONENT,
    COMMA,
    BITSLEFT,
    BITSRIGHT,
    EQUALS,
    LESSTHAN,
    GREATERTHAN,
    OPENPARENTHESES,
    CLOSEPARENTHESES,
    OPENBRACE,
    CLOSEBRACE,
    OPENBRACKET,
    CLOSEBRACKET,
    SEMICOLON,
    NEGATION,
    NOTEQUALS,
    DOUBLEEQUALS,
    GREATEREQUALS,
    LESSEQUALS,
    AND,
    OR,
    XOR,
    BITWISEAND,
    BITWISEOR,
    DOUBLEDOT,
    INVALID,
    DIGIT,
    FLOAT,
    STRING,
    LET,
    VAR,
    PUT,
    IF,
    ELSE,
    WHILE,
    FOR,
    BREAK,
    CONTINUE,
    FUNC,
    IN,
}
