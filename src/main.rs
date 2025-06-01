
use regex::RegexSet;
use std::error::Error;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().len() == 2 {
        if let Some(filename) = env::args().nth(1) {
            println!("Scanning file: {}", filename);
            let file_contents: String = fs::read_to_string(filename)?;
            let file_contents_nowhitespace: Vec<&str> = file_contents.split_whitespace().collect();
            let language_regexes = RegexSet::new(&[
                // regex for STRING datatype not possible because the normal split whitespace function will not ignore whitespace within a quote, pushdown automata needed
                // TODO: Implement string datatype or custom preprocessing (the latter is probably a more viable solution since spaces act a bit funky for other parameters too)
                r"[a-zA-Z]+[0-9a-zA-Z]*", // Identifier
                r"[0-9]+\.[0-9]+", // Float
                r"[0-9]+", // Integer
                r"\+",
                r"-",
                r"\*",
                r"/",
                r"^",
                r"=",
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
            let mut tokens: Vec<Token> = Vec::new();
            for current_item in file_contents_nowhitespace {
                let matches: Vec<usize>=  language_regexes.matches(current_item).into_iter().collect();
                tokens.push(match_to_token(matches, current_item));
            }
            for t in tokens {
                println!("{}", t.print_token());
            }
        }
        return Ok(());
    }
    Err("Usage: ./NProcScanner [input_file.np]".into())
}

fn match_to_token(m: Vec<usize>, current_item: &str) -> Token {
    if m.len() == 0  {
        return Token::Invalid
    }
    match m.get(0).unwrap() {
        0 => Token::Identifier(String::from(current_item)),
        1 => Token::FloatingPointNumber(current_item.parse().unwrap()),
        2 => Token::Digit(current_item.parse().unwrap()),
        3 => Token::Add,
        4 => Token::Subtract,
        5 => Token::Multiply,
        6 => Token::Divide,
        7 => Token::Exponent,
        8 => Token::Equals,
        9 => Token::OpenParentheses,
        10 => Token::CloseParentheses,
        11 => Token::OpenBrace,
        12 => Token::CloseBrace,
        13 => Token::OpenBracket,
        14 => Token::CloseBracket,
        15 => Token::Semicolon,
        16 => Token::ReservedWord(Keyword::INTEGER),
        17 => Token::ReservedWord(Keyword::FLOAT),
        18 => Token::ReservedWord(Keyword::STRING),
        19 => Token::ReservedWord(Keyword::LET),
        20 => Token::ReservedWord(Keyword::VAR),
        21 => Token::ReservedWord(Keyword::IF),
        22 => Token::ReservedWord(Keyword::ELSE),
        23 => Token::ReservedWord(Keyword::WHILE),
        24 => Token::ReservedWord(Keyword::FOR),
        25 => Token::ReservedWord(Keyword::IN),
        26 => Token::ReservedWord(Keyword::BREAK),
        27 => Token::ReservedWord(Keyword::CONTINUE),
        28 => Token::ReservedWord(Keyword::FUNCTION),
        _ => Token::Invalid
    }
}

enum Token {
    ReservedWord(Keyword),
    Identifier(String),
    FloatingPointNumber(f64),
    Digit(i128),
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Equals,
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
            Token::Equals => String::from("EQUALS"),
            Token::OpenParentheses => String::from("OPENPAREN"),
            Token::CloseParentheses => String::from("CLOSEPAREN"),
            Token::OpenBrace => String::from("OPENBRACE"),
            Token::OpenBracket => String::from("OPENBRACKET"),
            Token::CloseBracket => String::from("OPENBRACKET"),
            Token::CloseBrace => String::from("CLOSEBRACE"),
            Token::Semicolon => String::from("SEMICOLON"),
            Token::Digit(value) => format!("DIGIT: {}", value),
            Token::FloatingPointNumber(value) => format!("FPNUMBER: {}", value),
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