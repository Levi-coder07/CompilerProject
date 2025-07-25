// Importamos tipos necesarios desde el módulo del lexer y del AST
use crate::lexer::lexer::{Lexer, TokenType, LexerError, NumericHint, PunctuationKind};
use crate::ast::ASTNode;
use thiserror::Error;

// Definimos los distintos tipos de errores que pueden surgir durante el parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),
    
    #[error("Unexpected token: expected {expected:?}, found {found:?}")]
    UnexpectedToken {
        expected: String,
        found: TokenType,
    },
    
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    
    #[error("Invalid syntax: {message}")]
    InvalidSyntax { message: String },
}

// Estructura principal del parser, contiene un lexer y el token actual
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<TokenType>,
}

impl<'a> Parser<'a> {
    /// Crea una nueva instancia del parser a partir de una cadena de entrada
    pub fn new(input: &'a str) -> Result<Parser<'a>, ParseError> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token().ok();
        Ok(Parser {
            lexer,
            current_token,
        })
    }
    
    /// Punto de entrada principal del parser. Devuelve un nodo de programa con una lista de sentencias.
    pub fn parse(&mut self) -> Result<ASTNode, ParseError> {
        let mut statements = Vec::new();
        
        while let Some(ref token) = self.current_token {
            match token {
                TokenType::EOF => break,
                _ => {
                    let stmt = self.parse_expression_statement()?;
                    statements.push(stmt);
                    
                    // Skip optional semicolon
                    if let Some(TokenType::Punctuation { raw: ';', kind: PunctuationKind::Separator }) = &self.current_token {
                        self.advance()?;
                    }
                }
            }
        }
        
        Ok(ASTNode::Program { statements })
    }
    
    /// Avanza al siguiente token
    fn advance(&mut self) -> Result<(), ParseError> {
        self.current_token = match self.lexer.next_token() {
            Ok(token) => Some(token),
            Err(e) => return Err(ParseError::LexerError(e)),
        };
        Ok(())
    }
    
    /// Parsea una sentencia de expresión simple
    fn parse_expression_statement(&mut self) -> Result<ASTNode, ParseError> {
        let expr = self.parse_expression()?;
        Ok(ASTNode::ExpressionStatement {
            expression: Box::new(expr),
        })
    }
    
    /// Parsea una expresión completa (punto de entrada para precedencia)
    fn parse_expression(&mut self) -> Result<ASTNode, ParseError> {
        self.parse_assignment()
    }
    
    /// Parsea expresiones de asignación (con precedencia más baja)
    fn parse_assignment(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_or()?;
        
        // Verifica si es una asignación (`=`)
        if let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "=" {
                self.advance()?;
                let right = self.parse_assignment()?;
                left = ASTNode::Assignment {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
        }
        
        Ok(left)
    }
    
    /// Parsea operaciones OR (`||`)
    fn parse_or(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_and()?;
        
        while let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "||" {
                let operator = op.clone();
                self.advance()?;
                let right = self.parse_and()?;
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// Parsea operaciones AND (`&&`)
    fn parse_and(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_equality()?;
        
        while let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "&&" {
                let operator = op.clone();
                self.advance()?;
                let right = self.parse_equality()?;
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// Parsea comparaciones de igualdad (`==`, `!=`)
    fn parse_equality(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_comparison()?;
        
        while let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "==" || op == "!=" {
                let operator = op.clone();
                self.advance()?;
                let right = self.parse_comparison()?;
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// Parsea comparaciones relacionales (`<`, `<=`, `>`, `>=`)
    fn parse_comparison(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_addition()?;
        
        while let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "<" || op == ">" || op == "<=" || op == ">=" {
                let operator = op.clone();
                self.advance()?;
                let right = self.parse_addition()?;
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// Parsea operaciones aritméticas de suma y resta
    fn parse_addition(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_multiplication()?;
        
        while let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "+" || op == "-" {
                let operator = op.clone();
                self.advance()?;
                let right = self.parse_multiplication()?;
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// Parsea multiplicación y división
    fn parse_multiplication(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_unary()?;
        
        while let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "*" || op == "/" {
                let operator = op.clone();
                self.advance()?;
                let right = self.parse_unary()?;
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// Parsea expresiones unarias (`-`, `!`)
    fn parse_unary(&mut self) -> Result<ASTNode, ParseError> {
        if let Some(TokenType::Operator(ref op)) = &self.current_token {
            if op == "-" || op == "!" {
                let operator = op.clone();
                self.advance()?;
                let operand = self.parse_unary()?;
                return Ok(ASTNode::UnaryOp {
                    operator,
                    operand: Box::new(operand),
                });
            }
        }
        
        self.parse_primary()
    }
    
    /// Parsea expresiones primarias: números, cadenas, identificadores, llamadas, paréntesis
    fn parse_primary(&mut self) -> Result<ASTNode, ParseError> {
        match &self.current_token {
            Some(TokenType::Numero { raw, kind }) => {
                let value = raw.clone();
                let is_float = matches!(kind, NumericHint::Float);
                self.advance()?;
                Ok(ASTNode::Number { value, is_float })
            },
            Some(TokenType::Cadena(value)) => {
                let value = value.clone();
                self.advance()?;
                Ok(ASTNode::String { value })
            },
            Some(TokenType::Boolean(value)) => {
                let value = *value;
                self.advance()?;
                Ok(ASTNode::Boolean { value })
            },
            Some(TokenType::Identificador(name)) => {
                let name = name.clone();
                self.advance()?;
                
                // Check for function call
                if let Some(TokenType::Punctuation { raw: '(', kind: PunctuationKind::Open(_) }) = &self.current_token {
                    self.advance()?; // consume '('
                    let mut arguments = Vec::new();
                    
                    // Parse arguments
                    while let Some(ref token) = &self.current_token {
                        if let TokenType::Punctuation { raw: ')', kind: PunctuationKind::Close(_) } = token {
                            break;
                        }
                        
                        arguments.push(self.parse_expression()?);
                        
                        // Handle comma separation
                        if let Some(TokenType::Punctuation { raw: ',', kind: PunctuationKind::Separator }) = &self.current_token {
                            self.advance()?;
                        }
                    }
                    
                    // Consume closing parenthesis
                    if let Some(TokenType::Punctuation { raw: ')', kind: PunctuationKind::Close(_) }) = &self.current_token {
                        self.advance()?;
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            expected: "closing parenthesis".to_string(),
                            found: self.current_token.clone().unwrap_or(TokenType::EOF),
                        });
                    }
                    
                    Ok(ASTNode::FunctionCall { name, arguments })
                } else {
                    Ok(ASTNode::Identifier { name })
                }
            },
            Some(TokenType::Punctuation { raw: '(', kind: PunctuationKind::Open(_) }) => {
                self.advance()?; // consume '('
                let expression = self.parse_expression()?;
                
                // Expect closing parenthesis
                if let Some(TokenType::Punctuation { raw: ')', kind: PunctuationKind::Close(_) }) = &self.current_token {
                    self.advance()?;
                    Ok(ASTNode::Parenthesized {
                        expression: Box::new(expression),
                    })
                } else {
                    Err(ParseError::UnexpectedToken {
                        expected: "closing parenthesis".to_string(),
                        found: self.current_token.clone().unwrap_or(TokenType::EOF),
                    })
                }
            },
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: token.clone(),
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
} 