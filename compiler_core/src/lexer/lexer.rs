extern crate thiserror;
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO error")]
    FileIOError(#[from] io::Error),

    #[error("Unexpected Symbol")]
    MissingExpectedSymbol {
        expected: TokenType,
        found: Token
    },
    #[error("No se puede crear un numero debido a que el simbolo {raw:?} no es un simbolo numerico valido")]
    InvalidNumeric {
        raw: String,
    },
    #[error("No se puedo encontrar el simbolo abierto {open:?} para el simbolo cerrado {symbol:?}")]
    MissbalancedSymbols {
        symbol: char,
        open: char,
    },
    #[error("No se")]
    UnknwonSymbol {
        symbol: String,
    }
}

pub type Token = TokenType;
#[derive(Debug, Clone, PartialEq)]
pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind,
}
#[derive(Debug, Clone, PartialEq)]
pub enum NumericHint {
    Integer,
    Float,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Numeric {
    pub raw: String,
    pub kind: NumericHint,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    EOF,

    Punctuation{raw: char, kind: PunctuationKind},

    Operator(String),

    Identificador(String),

    Char(char),

    Numero{raw: String, kind: NumericHint},

    Cadena(String),
    Unknown(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationKind {
    Open(BalancingDepthType),

    Close(BalancingDepthType),

    Separator,
}

type BalancingDepthType = i32;
pub struct Lexer<'a> {
    pub cur_line: usize,
    pub cur_col: usize,

    pub position_offset: usize,

    chars: std::iter::Peekable<std::str::Chars<'a>>,
    
    balancing_state: std::collections::HashMap::<char, BalancingDepthType>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            cur_line: 1,
            cur_col: 0,
            position_offset: 0,
            chars: input.chars().peekable(),
            balancing_state: std::collections::HashMap::new(),
        }
    }
    fn map_balanced_state(c: &char) -> char {
        match c {
            '(' => ')',
            ')' => '(',
            '{' => '}',
            '}' => '{',
            '[' => ']',
            ']' => '[',
            _ => panic!("Unexpected character for balancing state: {}", c),
        }
    }
    fn push_open(&mut self, c: &char) -> BalancingDepthType {
        if let Some(pos ) = self.balancing_state.get_mut(&c) {
            *pos += 1;
            *pos - 1
        } else {
            self.balancing_state.insert(*c, 1);
            0
        }
    }

    fn pop_close(&mut self, c: &char) -> Result<BalancingDepthType, LexerError> {
        if let Some(pos) = self.balancing_state.get_mut(&Lexer::map_balanced_state(&c)) {
            if *pos >= 1 {
                *pos -= 1;
                Ok(*pos)
            } else {
                Err(LexerError::MissbalancedSymbols {
                    symbol: *c, open: Lexer::map_balanced_state(&c)
                })
            }
        } else {
            Err(LexerError::MissbalancedSymbols {
                    symbol: *c, open: Lexer::map_balanced_state(&c)
                })
        }
    }
    fn consume_digit(&mut self, raw:&String) -> Result<char, LexerError> {
        match self.chars.next() {
            Some(c) if !c.is_digit(10) => {
                Err(LexerError::InvalidNumeric { raw:raw.to_string() })
            },
            Some(c) => Ok(c),
            None => {Err(LexerError::InvalidNumeric { raw:raw.to_string()})},
        }
    }
    fn parse_number(&mut self, c: char) -> Result<TokenType, LexerError> {
        let mut seen_dot = false;
        let mut seen_e =false;
        let mut number = c.to_string();
        loop {
            match self.chars.peek() {
                Some(&next_char) if next_char.is_digit(10) => {
                    number.push(self.consume_char().unwrap());
                },
                Some(&'.') if !seen_dot && !seen_e => {
                    seen_dot = true;
                    number.push(self.consume_char().unwrap());
                },
                Some(&'e') | Some(&'E') if !seen_e => {
                    seen_e = true;
                    number.push(self.consume_char().unwrap());
                    if let Some(&next_char) = self.chars.peek() {
                        if next_char == '+' || next_char == '-' {
                            number.push(self.consume_char().unwrap());
                            self.consume_char();
                        }
                    }
                    self.consume_digit(&number)?;
                },
                Some(&next_char) if next_char.is_alphanumeric() => {
                    number.push(self.consume_char().unwrap());
                    return Err(LexerError::InvalidNumeric {
                        raw: number + &next_char.to_string(),
                    });
                },
                _ => { 
                    break Ok(TokenType::Numero {
                        raw: number,
                        kind: if seen_dot || seen_e {
                            NumericHint::Float
                        } else {
                            NumericHint::Integer
                        }
                    })
                }
            }
        }
       
    }

    fn parse_string(&mut self, _c: char) -> Result<TokenType, LexerError> {
        let mut string = String::new();
        loop {
            match self.chars.next() {
                Some('"') => break Ok(TokenType::Cadena(string)),
                Some('\\') => {
                    if let Some(escaped_char) = self.chars.next() {
                        string.push(escaped_char);
                    } else {
                        return Err(LexerError::UnknwonSymbol {
                            symbol: "Unterminated string literal".to_string(),
                        });
                    }
                },
                Some(c) => string.push(c),
                None => return Err(LexerError::UnknwonSymbol {
                    symbol: "Unterminated string literal".to_string(),
                }),
            }
        }
    }
    fn transform_to_type(&mut self,c: char) -> Result<TokenType, LexerError> {
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
            '0' ..= '9' => self.parse_number(c),
            '"' => { self.parse_string(c) },
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' => {
                // Handle single and multi-character operators
                let mut operator = c.to_string();
                if let Some(&next_char) = self.chars.peek() {
                    match (c, next_char) {
                        ('=', '=') | ('!', '=') | ('<', '=') | ('>', '=') |
                        ('&', '&') | ('|', '|') | ('+', '+') | ('-', '-') => {
                            operator.push(self.consume_char().unwrap());
                        },
                        _ => {}
                    }
                }
                Ok(TokenType::Operator(operator))
            },
            c if c.is_alphabetic() || c == '_' => {
                let mut identifier = c.to_string();
                while let Some(&next_char) = self.chars.peek() {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        identifier.push(self.consume_char().unwrap());
                    } else {
                        break;
                    }
                }
                Ok(TokenType::Identificador(identifier))
            },
            _ => Err(LexerError::UnknwonSymbol {
                symbol: c.to_string(),
            }),
        }
    }
    pub fn transfdor(&mut self) -> Result<Token, LexerError> {
        // Implementation of tokenization logic goes here
        unimplemented!()
    }
    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_whitespace();
        if let Some(c) = self.consume_char() {
            self.transform_to_type(c)
        } else {
            Ok(TokenType::EOF)
        }
    }
    pub fn consume_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.cur_col += 1;
                if c == '\n' {
                    self.cur_line += 1;
                    self.cur_col = 1;
                }
                self.position_offset += 1;
                Some(c)
            },
            None => None,
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
               break;
            }
            self.consume_char();
        }
    }
}