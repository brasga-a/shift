use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Punctuation Symbols (Single-char) / Símbolos de Pontuação (Caractere único) 
    LeftParen, RightParen,      // ( )
    LeftBrace, RightBrace,      // { }
    LeftBracket, RightBracket,  // [ ] 
    Comma, Question,            // , ?
    Dot, Range, Spread,         // . .. ...
    Colon, Semicolon,           // :  ;    

    // Mathematical and Logical Operators / Operadores Matemáticos e Lógicos
    Minus, Plus, Slash, Star, // -, +, /, *
    
    // Comparison and Assignment / Comparação e Atribuição
    Bang, BangEqual,       // ! !=
    Equal, EqualEqual,     // = ==
    Greater, GreaterEqual, // > >=
    Less, LessEqual,       // < <=
    Arrow,                 // =>     
    SimpleArrow,           // ->
    
    // Fast operations / Operações rapidas
    Increment, // '++'
    Decrement, // '--' 

    // Logical Operators and Bitwise / Operadores Lógicos e Bitwise 
    And,       // &&
    BitAnd,    // &
    OrLogical, // ||
    OrBitwise, // |

    // Literals / Literais
    Identifier(String), 
    String(String), 
    Integer(i64),
    Float(f64),

    // -------- Keywords --------

    // Modules / Modulos
    Import, //import
    Export, // export (pub in Rust)
    From,   // from
    As, // (import X as Y)

    // Data Types / Tipos de Dados
    Enum, // enum
    Type, // type (alias)
    Trait, // trait

    // Fluxo
    Match,

    // Control Structures / Estruturas de Controle
    If, Else, Return, While, For, Loop,

    // Declarations / Declarações
    Fn, Let, Const, Struct,
    
    // Const Values / Valores Constantes 
    True, False, Null, 

    // Asynchronous / Assíncrono
    Async, Await,

    // Shift Specific / Especificos do Shift
    Component, Server, Client, Signal, Derived, Effect, Prop,

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