use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, LeftAngle, RightAngle,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    
    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    // Literals
    Identifier(String), 
    String(String), 
    Integer(i64),
    Float(f64),
    Boolean(bool),

    // Keywords
    And, Class, Else, False, Function, For, If, Nil, Or,
    Print, Return, Super, This, True, While, Let, Const, Loop,
    
    // Adicionar mais depois

    Eof,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_type, self.lexeme)
    }
}