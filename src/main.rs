use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().len() == 2 {
        if let Some(filename) = env::args().nth(1) {
            println!("Scanning file: {}", filename);
            let file_contents: Vec<char> = fs::read_to_string(filename)?.chars().collect();
            let tokens: Vec<Token> = tokenize_input(file_contents);
            for t in tokens {
                println!("{:#?}", t);
            }
        }
        return Ok(());
    }
    Err("Usage: ./NProcScanner [input_file.np]".into())
}

fn tokenize_input(file_contents: Vec<char>) -> Vec<Token> {
    let mut current: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        if current >= file_contents.len() {
            break;
        }
        match file_contents.get(current).unwrap() {
            ';' => {
                tokens.push(Token::SEMICOLON);
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
            '*' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '*' => tokens.push(Token::EXPONENT),
                    _ => tokens.push(Token::MULTIPLY)
                }
                tokens.push(Token::MULTIPLY);
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
                // Handle digits and floating point numbers
                let mut buf: String = String::new();
                while file_contents.get(current).unwrap().is_digit(10) {
                    buf.push(*file_contents.get(current).unwrap());
                    current += 1;
                }
                tokens.push(Token::DIGITLITERAL(buf.parse().unwrap()));
            }
            _ => tokens.push(Token::INVALID),
        }
    }
    tokens
}

#[derive(Debug)]
enum Token {
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
