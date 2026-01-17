use crate::token::{Token, TokenType};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
  Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
  },

  Grouping {
    expression: Box<Expr>
  },

  Literal {
    value: LiteralValue,
  },

  Unary {
    operator: Token,
    right: Box<Expr>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
  Integer(i64),
  Float(f64),
  String(String),
  Boolean(bool),
  Null,
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expr::Binary { left, operator, right } => {
        write!(f, "({} {} {})", operator.lexeme, left , right)
      },
      Expr::Grouping { expression } => {
          write!(f, "(group {})", expression)
      },
      Expr::Literal { value } => {
          write!(f, "{}", value)
      },
      Expr::Unary { operator, right } => {
          write!(f, "({} {})", operator.lexeme, right)
      },
    }
  }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::String(s) => write!(f, "\"{}\"", s), // Strings com aspas
            LiteralValue::Integer(i) => write!(f, "{}", i),
            LiteralValue::Float(fl) => write!(f, "{}", fl),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
        }
    }
}