//! Lexer implementation with improved readability, idiomatic Rust, and error handling

extern crate thiserror;

use std::collections::HashMap;
use std::io;
use std::iter::Peekable;
use std::str::Chars;

use serde::{Deserialize, Serialize};
use thiserror::Error;

// =====================
// Error Definitions
// =====================

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO error")]
    FileIOError(#[from] io::Error),

    #[error("Unexpected symbol: expected {expected:?}, found {found:?}")]
    MissingExpectedSymbol {
        expected: TokenType,
        found: Token,
    },

    #[error("Invalid numeric symbol: {raw:?}")]
    InvalidNumeric { raw: String },

    #[error("Unmatched opening symbol {open:?} for closing symbol {symbol:?}")]
    MissbalancedSymbols { symbol: char, open: char },

    #[error("Unknown symbol: {symbol}")]
    UnknownSymbol { symbol: String },
}

// =====================
// Token and AST Structs
// =====================

pub type Token = TokenType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumericHint {
    Integer,
    Float,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Numeric {
    pub raw: String,
    pub kind: NumericHint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    EOF,
    Punctuation { raw: char, kind: PunctuationKind },
    Operator(String),
    Identificador(String),
    Char(char),
    Numero { raw: String, kind: NumericHint },
    Cadena(String),
    Boolean(bool),
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PunctuationKind {
    Open(BalancingDepthType),
    Close(BalancingDepthType),
    Separator,
}

type BalancingDepthType = i32;
type CharIter<'a> = Peekable<Chars<'a>>;

// =====================
// Lexer Implementation
// =====================

pub struct Lexer<'a> {
    pub cur_line: usize,
    pub cur_col: usize,
    pub position_offset: usize,
    chars: CharIter<'a>,
    balancing_state: HashMap<char, BalancingDepthType>,
}

impl<'a> Lexer<'a> {
    /// Creates a new Lexer from input string
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            cur_line: 1,
            cur_col: 0,
            position_offset: 0,
            chars: input.chars().peekable(),
            balancing_state: HashMap::new(),
        }
    }

    /// Maps an opening or closing symbol to its matching pair
    fn map_balanced_state(c: &char) -> char {
        match c {
            '(' => ')', ')' => '(',
            '{' => '}', '}' => '{',
            '[' => ']', ']' => '[',
            _ => panic!("Unexpected character for balancing state: {}", c),
        }
    }

    /// Pushes an opening symbol and increases its depth
    fn push_open(&mut self, c: &char) -> BalancingDepthType {
        let entry = self.balancing_state.entry(*c).or_insert(0);
        let current = *entry;
        *entry += 1;
        current
    }

    /// Pops a closing symbol and validates it against expected opening symbol
    fn pop_close(&mut self, c: &char) -> Result<BalancingDepthType, LexerError> {
        let open = Self::map_balanced_state(c);
        match self.balancing_state.get_mut(&open) {
            Some(depth) if *depth > 0 => {
                *depth -= 1;
                Ok(*depth)
            }
            _ => Err(LexerError::MissbalancedSymbols { symbol: *c, open }),
        }
    }

    /// Consumes one digit and validates it
    fn consume_digit(&mut self, raw: &str) -> Result<char, LexerError> {
        match self.chars.next() {
            Some(c) if c.is_ascii_digit() => Ok(c),
            Some(_) | None => Err(LexerError::InvalidNumeric { raw: raw.to_string() }),
        }
    }

    /// Parses a numeric literal, including integers and floats with optional exponent
    fn parse_number(&mut self, c: char) -> Result<TokenType, LexerError> {
        let mut seen_dot = false;
        let mut seen_e = false;
        let mut number = c.to_string();

        while let Some(&next) = self.chars.peek() {
            match next {
                d if d.is_ascii_digit() => number.push(self.consume_char().unwrap()),
                '.' if !seen_dot && !seen_e => {
                    seen_dot = true;
                    number.push(self.consume_char().unwrap());
                }
                'e' | 'E' if !seen_e => {
                    seen_e = true;
                    number.push(self.consume_char().unwrap());
                    if matches!(self.chars.peek(), Some('+' | '-')) {
                        number.push(self.consume_char().unwrap());
                    }
                    self.consume_digit(&number)?;
                }
                a if a.is_alphabetic() => {
                    number.push(self.consume_char().unwrap());
                    return Err(LexerError::InvalidNumeric { raw: number });
                }
                _ => break,
            }
        }

        Ok(TokenType::Numero {
            raw: number,
            kind: if seen_dot || seen_e {
                NumericHint::Float
            } else {
                NumericHint::Integer
            },
        })
    }

    /// Parses a string literal with support for escape sequences
    fn parse_string(&mut self, _c: char) -> Result<TokenType, LexerError> {
        let mut string = String::new();

        while let Some(c) = self.chars.next() {
            match c {
                '"' => return Ok(TokenType::Cadena(string)),
                '\\' => {
                    if let Some(escaped) = self.chars.next() {
                        string.push(escaped);
                    } else {
                        return Err(LexerError::UnknownSymbol {
                            symbol: "Unterminated escape sequence".to_string(),
                        });
                    }
                }
                other => string.push(other),
            }
        }

        Err(LexerError::UnknownSymbol {
            symbol: "Unterminated string literal".to_string(),
        })
    }

    /// Maps a character to its corresponding token type
    fn transform_to_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' | '{' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Open(self.push_open(&c)),
            }),
            ')' | ']' | '}' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Close(self.pop_close(&c)?),
            }),
            ',' | ';' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Separator,
            }),
            '0'..='9' => self.parse_number(c),
            '"' => self.parse_string(c),
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' => {
                let mut operator = c.to_string();
                if let Some(&next) = self.chars.peek() {
                    if matches!((c, next), 
                        ('=', '=') | ('!', '=') | ('<', '=') | ('>', '=') |
                        ('&', '&') | ('|', '|') | ('+', '+') | ('-', '-')) 
                    {
                        operator.push(self.consume_char().unwrap());
                    }
                }
                Ok(TokenType::Operator(operator))
            }
            a if a.is_alphabetic() || a == '_' => {
                let mut ident = a.to_string();
                while matches!(self.chars.peek(), Some(c) if c.is_alphanumeric() || *c == '_') {
                    ident.push(self.consume_char().unwrap());
                }
                match ident.as_str() {
                    "true" => Ok(TokenType::Boolean(true)),
                    "false" => Ok(TokenType::Boolean(false)),
                    _ => Ok(TokenType::Identificador(ident)),
                }
            }
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string() }),
        }
    }

    /// Returns the next token in the stream
    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_whitespace();
        match self.consume_char() {
            Some(c) => self.transform_to_type(c),
            None => Ok(TokenType::EOF),
        }
    }

    /// Consumes a character and updates cursor position
    pub fn consume_char(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            self.position_offset += 1;
            if c == '\n' {
                self.cur_line += 1;
                self.cur_col = 1;
            } else {
                self.cur_col += 1;
            }
            c
        })
    }

    /// Skips all whitespace characters
    fn skip_whitespace(&mut self) {
        while matches!(self.chars.peek(), Some(c) if c.is_whitespace()) {
            self.consume_char();
        }
    }

    /// Unit test helper to tokenize full input into a vector
    pub fn tokenize_all(&mut self) -> Result<Vec<TokenType>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            if matches!(token, TokenType::EOF) {
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}

// =====================
// Unit Tests
// =====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_number() {
        let mut lexer = Lexer::new("123");
        let token = lexer.next_token().unwrap();
        match token {
            TokenType::Numero { raw, kind } => {
                assert_eq!(raw, "123");
                assert_eq!(kind, NumericHint::Integer);
            }
            _ => panic!("Expected number token"),
        }
    }

    #[test]
    fn test_boolean_true() {
        let mut lexer = Lexer::new("true");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, TokenType::Boolean(true));
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"hello\"");
        let token = lexer.next_token().unwrap();
        assert_eq!(token, TokenType::Cadena("hello".to_string()));
    }

    #[test]
    fn test_peek_and_next_token() {
        let mut lexer = Lexer::new("42");
        let peeked = lexer.peek_token().unwrap();
        let next = lexer.next_token().unwrap();
        assert_eq!(peeked, next);
    }

    #[test]
    fn test_balanced_symbols() {
        let mut lexer = Lexer::new("({[]})");
        let tokens = lexer.tokenize_all().unwrap();
        assert_eq!(tokens.len(), 6);
    }
}
