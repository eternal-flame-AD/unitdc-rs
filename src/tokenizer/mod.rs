// Copyright 2024 eternal-flame-AD <yume@yumechi.jp>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    fmt::Display,
    io::{BufReader, Read},
};
pub mod parsing;
pub mod token;

use thiserror::Error;

use token::Token;

use self::parsing::parse_bigrational;

pub struct Tokenizer<R>
where
    R: std::io::Read,
{
    input: BufReader<R>,
    unread_buffer: Option<char>,
    cursor: ReaderCursor,
}

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("IO Error: {0}")]
    IOError(std::io::Error),
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
}

#[derive(Debug, Clone, Copy)]
pub struct ReaderCursor {
    pub line: usize,
    pub column: usize,
}

impl Display for ReaderCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for ReaderCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl ReaderCursor {
    pub fn new() -> Self {
        ReaderCursor { line: 1, column: 1 }
    }
}

impl<R: std::io::Read> Tokenizer<R> {
    pub fn new(input: R) -> Self {
        Tokenizer {
            input: BufReader::new(input),
            unread_buffer: None,
            cursor: ReaderCursor::new(),
        }
    }
    fn next_char(&mut self) -> Result<Option<char>, std::io::Error> {
        if let Some(ch) = self.unread_buffer {
            self.unread_buffer = None;
            return Ok(Some(ch));
        }
        let mut buf = [0; 1];
        match self.input.read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(_) => {
                if buf[0] == b'\n' {
                    self.cursor.line += 1;
                    self.cursor.column = 1;
                } else {
                    self.cursor.column += 1;
                }
                Ok(Some(buf[0] as char))
            }
            Err(e) => Err(e),
        }
    }
    fn next_char_non_whitespace(&mut self) -> Result<Option<char>, std::io::Error> {
        loop {
            match self.next_char()? {
                Some(ch) if ch.is_whitespace() => {}
                ch => return Ok(ch),
            }
        }
    }
    fn unread_char(&mut self, ch: char) {
        if self.unread_buffer.is_some() {
            panic!("Cannot unread more than one character");
        }
        self.unread_buffer = Some(ch);
    }
    pub fn get_cursor(&self) -> ReaderCursor {
        self.cursor
    }
    pub fn parse_next_token(&mut self) -> Result<Option<Token>, TokenizerError> {
        let mut buf = String::new();
        let ch = self
            .next_char_non_whitespace()
            .map_err(TokenizerError::IOError)?;
        match ch {
            Some('0'..='9' | '_') => {
                buf.push(ch.unwrap());
                while let Some(c) = self.next_char().map_err(TokenizerError::IOError)? {
                    match c {
                        '0'..='9' => buf.push(c),
                        '.' | 'e' | 'E' | '_' | '-' => buf.push(c),
                        _ => {
                            self.unread_char(c);
                            break;
                        }
                    }
                }
                Ok(Some(Token::Number(parse_bigrational(&buf)?)))
            }
            Some('(') => {
                while let Some(c) = self.next_char().map_err(TokenizerError::IOError)? {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '/' | '*' | '_' => buf.push(c),
                        ')' => break,
                        _ => return Err(TokenizerError::InvalidCharacter(c)),
                    }
                }
                Ok(Some(Token::Unit(buf)))
            }
            Some('@') => {
                while let Some(c) = self.next_char().map_err(TokenizerError::IOError)? {
                    match c {
                        'a'..='z' | 'A'..='Z' | '_' => buf.push(c),
                        '(' => {
                            break;
                        }
                        _ => return Err(TokenizerError::InvalidCharacter(c)),
                    }
                }
                let macro_name = buf.clone();
                buf.clear();
                while let Some(c) = self.next_char().map_err(TokenizerError::IOError)? {
                    match c {
                        ')' => break,
                        _ => buf.push(c),
                    }
                }
                Ok(Some(Token::MacroInvoke((macro_name, buf))))
            }
            Some('+') => Ok(Some(Token::Add)),
            Some('-') => Ok(Some(Token::Sub)),
            Some('*') => Ok(Some(Token::Mul)),
            Some('/') => Ok(Some(Token::Div)),
            Some('p') => Ok(Some(Token::Operator('p'))),
            Some('n') => Ok(Some(Token::Operator('n'))),
            Some('f') => Ok(Some(Token::Operator('f'))),
            Some('c') => Ok(Some(Token::Operator('c'))),
            Some('d') => Ok(Some(Token::Operator('d'))),
            Some('r') => Ok(Some(Token::Operator('r'))),
            Some('s') => Ok(Some(Token::Operator('s'))),
            Some('U') => Ok(Some(Token::Operator('U'))),
            Some('#') => {
                while let Some(c) = self.next_char().map_err(TokenizerError::IOError)? {
                    match c {
                        '\r' | '\n' => break,
                        _ => buf.push(c),
                    }
                }
                Ok(Some(Token::Comment(buf)))
            }
            Some('>') => {
                while let Some(c) = self.next_char().map_err(TokenizerError::IOError)? {
                    match c {
                        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => buf.push(c),
                        _ => {
                            self.unread_char(c);
                            break;
                        }
                    }
                }
                Ok(Some(Token::VarStore(buf)))
            }
            Some('<') => {
                while let Some(c) = self.next_char().map_err(TokenizerError::IOError)? {
                    match c {
                        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => buf.push(c),
                        _ => {
                            self.unread_char(c);
                            break;
                        }
                    }
                }
                Ok(Some(Token::VarRecall(buf)))
            }
            None => Ok(None),
            _ => Err(TokenizerError::InvalidCharacter(ch.unwrap())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::BigRational;
    use num_traits::FromPrimitive;

    #[test]
    fn test_tokenizer() {
        let input = "1 2e3+ 3.4e5* (g) 4.5(ml) / 6_789 + 3.14".as_bytes();
        let mut tokenizer = Tokenizer::new(input);
        assert!(tokenizer
            .parse_next_token()
            .unwrap()
            .unwrap()
            .roughly_eq(&Token::Number(
                BigRational::from_i64(1).expect("Failed to parse number")
            )));
        assert!(tokenizer
            .parse_next_token()
            .unwrap()
            .unwrap()
            .roughly_eq(&Token::Number(
                BigRational::from_i64(2e3 as i64).expect("Failed to parse number")
            )));
        assert_eq!(tokenizer.parse_next_token().unwrap().unwrap(), Token::Add);
        assert!(tokenizer
            .parse_next_token()
            .unwrap()
            .unwrap()
            .roughly_eq(&Token::Number(
                BigRational::from_f64(3.4e5).expect("Failed to parse number")
            )));
        assert_eq!(tokenizer.parse_next_token().unwrap().unwrap(), Token::Mul);
        assert_eq!(
            tokenizer.parse_next_token().unwrap().unwrap(),
            Token::Unit("g".to_string())
        );
        assert!(tokenizer
            .parse_next_token()
            .unwrap()
            .unwrap()
            .roughly_eq(&Token::Number(
                BigRational::from_f64(4.5).expect("Failed to parse number")
            )));
        assert_eq!(
            tokenizer.parse_next_token().unwrap().unwrap(),
            Token::Unit("ml".to_string())
        );
        assert_eq!(tokenizer.parse_next_token().unwrap().unwrap(), Token::Div);
        assert!(tokenizer
            .parse_next_token()
            .unwrap()
            .unwrap()
            .roughly_eq(&Token::Number(
                BigRational::from_i64(6789).expect("Failed to parse number")
            )));
        assert_eq!(tokenizer.parse_next_token().unwrap().unwrap(), Token::Add);
        assert!(tokenizer
            .parse_next_token()
            .unwrap()
            .unwrap()
            .roughly_eq(&Token::Number(
                #[allow(clippy::approx_constant)]
                BigRational::from_f64(3.14).expect("Failed to parse number")
            )));
    }
}
