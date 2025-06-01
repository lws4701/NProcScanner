
fn main() {
    println!("Hello, world!");
}

enum Token {
    ReservedWord(Keyword),
    Identifier(String),
    Str(String),
    FloatingPointNumber(f64),
    Digit(i128),
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    OpenParentheses,
    CloseParentheses,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

impl Token {
    fn print_token(&self) -> String {
        match self {
            Token::ReservedWord(value) => value.print_keyword(),
            Token::Add => String::from("ADD"),
            Token::Subtract => String::from("SUBTRACT"),
            Token::Multiply => String::from("MULTIPLY"),
            Token::Divide => String::from("DIVIDE"),
            Token::Exponent => String::from("EXPONENT"),
            Token::OpenParentheses => String::from("OPENPAREN"),
            Token::CloseParentheses => String::from("CLOSEPAREN"),
            Token::OpenBrace => String::from("OPENBRACE"),
            Token::CloseBrace => String::from("CLOSEBRACE"),
            Token::Semicolon => String::from("SEMICOLON"),
            Token::Digit(value) => format!("DIGIT: {}", value),
            Token::FloatingPointNumber(value) => format!("FPNUMBER: {}", value),
            Token::Str(value) => format!("STRING: {}", value),
            Token::Identifier(value) => format!("IDENTIFIER {}", value)
        }
    }
}

#[derive(Debug)]
enum Keyword {
    INTEGER,
    FLOAT,
    LET,
    VAR,
    IF,
    ELSE,
    WHILE,
    FOR,
    BREAK,
    CONTINUE,
    FUNCTION
}

impl Keyword {
    fn print_keyword(&self) -> String {
        match self {
                Keyword::BREAK => String::from("BREAK"),
                Keyword::CONTINUE => String::from("CONTINUE"),
                Keyword::ELSE => String::from("ELSE"),
                Keyword::FLOAT => String::from("FLOAT"),
                Keyword::FOR => String::from("FOR"),
                Keyword::FUNCTION => String::from("FUNCTION"),
                Keyword::IF => String::from("IF"),
                Keyword::INTEGER => String::from("INTEGER"),
                Keyword::LET => String::from("LET"),
                Keyword::VAR => String::from("VAR"),
                Keyword::WHILE => String::from("WHILE")
        }
    }
}