
use regex::RegexSet;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let language_regexes = RegexSet::new(&[
        r#""[0-9a-zA-Z]+""#,
        r"[a-zA-Z]+[0-9a-zA-Z]*",
        r"[0-9]+.[0-9]+",
        r"[0-9]+",
        r"\+",
        r"-",
        r"\*",
        r"/",
        r"^",
        r"\(",
        r"\)",
        r"\{",
        r"\}",
        r"\[",
        r"\]",
        r";",
        r"int",
        r"float",
        r"str",
        r"let",
        r"var",
        r"if",
        r"else",
        r"while",
        r"for",
        r"in",
        r"break",
        r"continue",
        r"func"
        ]).unwrap();
    println!("Hello, world!");
    Ok(())
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
    OpenBracket,
    CloseBracket,
    Semicolon,
    Invalid,
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
            Token::OpenBracket => String::from("OPENBRACKET"),
            Token::CloseBracket => String::from("OPENBRACKET"),
            Token::CloseBrace => String::from("CLOSEBRACE"),
            Token::Semicolon => String::from("SEMICOLON"),
            Token::Digit(value) => format!("DIGIT: {}", value),
            Token::FloatingPointNumber(value) => format!("FPNUMBER: {}", value),
            Token::Str(value) => format!("STRING: {}", value),
            Token::Identifier(value) => format!("IDENTIFIER {}", value),
            _ => String::from("INVALIDTOKEN")
        }
    }
}

#[derive(Debug)]
enum Keyword {
    INTEGER,
    FLOAT,
    STRING,
    LET,
    VAR,
    IF,
    ELSE,
    WHILE,
    FOR,
    BREAK,
    CONTINUE,
    FUNCTION,
    IN
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
                Keyword::WHILE => String::from("WHILE"),
                Keyword::IN => String::from("IN"),
                Keyword::STRING => String::from("STRINGDEF")
        }
    }
}