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
pub enum TokenType {
    EOF,

    Punctuation{raw: char, kind: PunctuationKind},

    Operator(String),

    Identifier(String),

    Char(char),

    Numeric(String),

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
    fn transform_to_type(&mut self,c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Open(self.push_open(&c)),
            }),
            ')' | ']' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Close(self.pop_close(&c)?),
            }),
           
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