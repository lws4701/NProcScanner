/*
Copyright 2025 Lewis Smith

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS” AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

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
            '.' => {
                tokens.push(Token::DOT);
                current += 1;
            }
            // Handle multi-character tokens
            '*' => {
                current += 1;
                match file_contents.get(current).unwrap() {
                    '*' => tokens.push(Token::EXPONENT),
                    _ => tokens.push(Token::MULTIPLY),
                }
                current += 1;
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
                    "until" => tokens.push(Token::UNTIL),
                    "continue" => tokens.push(Token::CONTINUE),
                    "func" => tokens.push(Token::FUNC),
                    "in" => tokens.push(Token::IN),
                    "inf" => tokens.push(Token::INF),
                    "nil" => tokens.push(Token::NIL),
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
    INVALID,
    DIGIT,
    FLOAT,
    STRING,
    LET,
    VAR,
    PUT,
    IF,
    ELSE,
    UNTIL,
    WHILE,
    FOR,
    BREAK,
    CONTINUE,
    FUNC,
    IN,
    INF,
    NIL,
}

#[cfg(test)]
mod test {
    use crate::lexer::{Token, tokenize_input};

    #[test]
    fn test_basic_functionality() {
        let test_string: Vec<char> = String::from(
            "func main(string[] args) { \
          var digit hamster = 5; \
          hamster = hamster << 1; \
          var float cheese = 9.01; \
          for i in 0 until hamster { \
            put (i ^ hamster) * cheese; \
          put \"It works!\"; \
          for i in 3 until 7 { \
            if (i == 4) { \
              continue; \
            else if i == 6 { \
              break; \
            } \
            else { \
              put i; \
            } \
          } \
          cheese = cheese ^ cheese; \
          let digit foo = 9009034; \
          } \
        ",
        )
        .chars()
        .collect();
        let tokens: Vec<Token> = tokenize_input(test_string);
        assert_eq!(tokens.len(), 94);
    }
}
