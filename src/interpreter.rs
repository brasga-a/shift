use std::fmt;

use crate::expr::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
  Integer(i64),
  Float(f64),
  String(String),
  Boolean(bool),
  Null,
}

impl fmt::Display for RuntimeValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RuntimeValue::Integer(i) => write!(f, "{}", i),
      RuntimeValue::Float(fl) => write!(f, "{}", fl),
      RuntimeValue::String(s) => write!(f, "{}", s),
      RuntimeValue::Boolean(b) => write!(f, "{}", b),
      RuntimeValue::Null => write!(f, "null"),
    }
  }
}

pub struct Interpreter;

impl Interpreter {
  pub fn interpret(&self, expression: &Expr) {
    match self.evaluate(expression) {
      Ok(value) => { println!("{}", value);},

      Err(error) => {
        eprintln!("Runtime Error: {}", error);
      }
    }
  }

  fn evaluate(&self, expr: &Expr) -> Result<RuntimeValue, String> {
    match expr {
      //Literals
      Expr::Literal { value } => {
        let runtime_val = match value {
          LiteralValue::Integer(i) => RuntimeValue::Integer(*i),
          LiteralValue::Float(f) => RuntimeValue::Float(*f),
          LiteralValue::String(s) => RuntimeValue::String(s.clone()),
          LiteralValue::Boolean(b) => RuntimeValue::Boolean(*b),
          LiteralValue::Null => RuntimeValue::Null,
        };

        Ok(runtime_val)
      },

      // Grouping

      Expr::Grouping { expression } => {
        self.evaluate(expression)
      },
      // unary

      Expr::Unary { operator, right } => {
        let r = self.evaluate(right)?;

        self.evaluate_unary(operator.token_type.clone(), r)
      },

      // binary

      Expr::Binary { left, operator, right } => {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;
        self.evaluate_binary(l, operator.token_type.clone(), r)
      }

    }
  }

  fn evaluate_unary(&self, operator: TokenType, right: RuntimeValue) -> Result<RuntimeValue, String> {
    match operator {
          TokenType::Minus => {
            match right {
              RuntimeValue::Integer(i) => Ok(RuntimeValue::Integer(-i)),
              RuntimeValue::Float(f) => Ok(RuntimeValue::Float(-f)),
              _ => Err("Need be a number".to_string())
            }
          },
          TokenType::Bang => {
            Ok(RuntimeValue::Boolean(!self.is_truthy(&right)))
          },
          _ => Err("Invalid operator".to_string())
        }
  }

  fn evaluate_binary(&self, left: RuntimeValue, operator: TokenType, right: RuntimeValue) -> Result<RuntimeValue, String> {
    match operator {
        // --- Aritmética (Menos, Vezes, Divisão) ---
        TokenType::Minus | TokenType::Slash | TokenType::Star => {
            match (left, right) {
                (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => match operator {
                    TokenType::Minus => Ok(RuntimeValue::Integer(l - r)),
                    TokenType::Star => Ok(RuntimeValue::Integer(l * r)),
                    TokenType::Slash => {
                        if r == 0 { return Err("Divisão por zero".to_string()); }
                        Ok(RuntimeValue::Integer(l / r)) // Divisão Inteira
                    },
                    _ => unreachable!(),
                },
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => match operator {
                    TokenType::Minus => Ok(RuntimeValue::Float(l - r)),
                    TokenType::Star => Ok(RuntimeValue::Float(l * r)),
                    TokenType::Slash => Ok(RuntimeValue::Float(l / r)),
                    _ => unreachable!(),
                },
                // Mistos (Int e Float) -> Promove para Float
                (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                    let l = l as f64;
                    match operator {
                        TokenType::Minus => Ok(RuntimeValue::Float(l - r)),
                        TokenType::Star => Ok(RuntimeValue::Float(l * r)),
                        TokenType::Slash => Ok(RuntimeValue::Float(l / r)),
                        _ => unreachable!(),
                    }
                },
                (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                    let r = r as f64;
                    match operator {
                        TokenType::Minus => Ok(RuntimeValue::Float(l - r)),
                        TokenType::Star => Ok(RuntimeValue::Float(l * r)),
                        TokenType::Slash => Ok(RuntimeValue::Float(l / r)),
                        _ => unreachable!(),
                    }
                },
                _ => Err("Math operations require numbers".to_string())
            }
        },

        // --- Soma e Concatenação ---
        TokenType::Plus => {
            match (left, right) {
                (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => Ok(RuntimeValue::Integer(l + r)),
                (RuntimeValue::Float(l), RuntimeValue::Float(r))     => Ok(RuntimeValue::Float(l + r)),
                (RuntimeValue::Integer(l), RuntimeValue::Float(r))   => Ok(RuntimeValue::Float(l as f64 + r)),
                (RuntimeValue::Float(l), RuntimeValue::Integer(r))   => Ok(RuntimeValue::Float(l + r as f64)),
                (RuntimeValue::String(l), RuntimeValue::String(r))   => Ok(RuntimeValue::String(format!("{}{}", l, r))),
                _ => Err("The operator '+' requires to numbers or to strings".to_string())
            }
        },

        // --- Comparação (>, <, >=, <=) ---
        TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
            match (left, right) {
                (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => match operator {
                    TokenType::Greater      => Ok(RuntimeValue::Boolean(l > r)),
                    TokenType::GreaterEqual => Ok(RuntimeValue::Boolean(l >= r)),
                    TokenType::Less         => Ok(RuntimeValue::Boolean(l < r)),
                    TokenType::LessEqual    => Ok(RuntimeValue::Boolean(l <= r)),
                    _ => unreachable!(),
                },
                (RuntimeValue::Float(l), RuntimeValue::Float(r)) => match operator {
                    TokenType::Greater      => Ok(RuntimeValue::Boolean(l > r)),
                    TokenType::GreaterEqual => Ok(RuntimeValue::Boolean(l >= r)),
                    TokenType::Less         => Ok(RuntimeValue::Boolean(l < r)),
                    TokenType::LessEqual    => Ok(RuntimeValue::Boolean(l <= r)),
                    _ => unreachable!(),
                },
                // Mistos
                (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                    let l = l as f64;
                    match operator {
                        TokenType::Greater      => Ok(RuntimeValue::Boolean(l > r)),
                        TokenType::GreaterEqual => Ok(RuntimeValue::Boolean(l >= r)),
                        TokenType::Less         => Ok(RuntimeValue::Boolean(l < r)),
                        TokenType::LessEqual    => Ok(RuntimeValue::Boolean(l <= r)),
                        _ => unreachable!(),
                    }
                },
                (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                    let r = r as f64;
                    match operator {
                        TokenType::Greater      => Ok(RuntimeValue::Boolean(l > r)),
                        TokenType::GreaterEqual => Ok(RuntimeValue::Boolean(l >= r)),
                        TokenType::Less         => Ok(RuntimeValue::Boolean(l < r)),
                        TokenType::LessEqual    => Ok(RuntimeValue::Boolean(l <= r)),
                        _ => unreachable!(),
                    }
                },
                _ => Err("Operadores de comparação requerem números".to_string())
            }
        },

        // --- Igualdade ---
        TokenType::EqualEqual => Ok(RuntimeValue::Boolean(left == right)),
        TokenType::BangEqual  => Ok(RuntimeValue::Boolean(left != right)),

        _ => Err("Operador desconhecido ou não suportado em binários".to_string())
    }
  }

  fn is_truthy(&self, val: &RuntimeValue) -> bool {
    match val {
      RuntimeValue::Null => false,
      RuntimeValue::Boolean(b) => *b,
      _ => true,
    }
  }

}