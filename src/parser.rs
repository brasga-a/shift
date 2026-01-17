use core::fmt;

use crate::token::{Token, TokenType};
use crate::expr::{Expr, LiteralValue};

#[derive(Debug, Clone)]
pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
}

// Isso deve ir em um arquivo separado posteriormente! ~ Brasga
#[derive(Debug, Clone)]
pub struct ParserError {
  pub token: Token,
  pub message:String,
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[Line {}] Error at '{}': {}", 
      self.token.line,
      self.token.lexeme,
      self.message
    )
  }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
      Self {
        tokens,
        current: 0,
      }
    }
    
    fn primary(&mut self) -> Result<Expr, ParserError> {
      let token = self.peek();

      match token.token_type {
        TokenType::False              => { self.advance(); Ok(Expr::Literal { value: LiteralValue::Boolean(false) })},
        TokenType::True               => { self.advance(); Ok(Expr::Literal { value: LiteralValue::Boolean(true) })},
        TokenType::Null               => { self.advance(); Ok(Expr::Literal { value: LiteralValue::Null })},
        TokenType::Integer(i)    => { self.advance(); Ok(Expr::Literal { value: LiteralValue::Integer(i) })},
        TokenType::Float(f)      => { self.advance(); Ok(Expr::Literal { value: LiteralValue::Float(f) })},
        TokenType::String(s)  => { self.advance(); Ok(Expr::Literal { value: LiteralValue::String(s) })},

        TokenType::LeftParen => {
          self.advance();
          let expr = self.expression()?;
          self.consume(TokenType::RightParen, "Expect ')' after expression")?;

          Ok(Expr::Grouping { expression: Box::new(expr) })
        },
        _ => {
          let token = self.peek();
          Err(self.error(token, "Expect expression."))
        }
      }
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
      if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
        let operator: Token = self.previous();
        let right: Expr = self.unary()?;
        return Ok(Expr::Unary { operator, right: Box::new(right) })
      }

      self.primary()
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
      let mut expr = self.unary()?;

      while self.match_token(&[TokenType::Slash, TokenType::Star]) {
        let operator: Token = self.previous();
        let right: Expr = self.unary()?;
        expr = Expr::Binary { 
          left: Box::new(expr), 
          operator, 
          right: Box::new(right) 
        };
      }

      Ok(expr)
    }

    fn term (&mut self) -> Result<Expr, ParserError> {
      let mut expr = self.factor()?;

      while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
        let operator: Token = self.previous();
        let right: Expr = self.factor()?;
        expr = Expr::Binary { 
          left: Box::new(expr), 
          operator, 
          right: Box::new(right) 
        };
      }

      Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError>{
      let mut expr = self.term()?;

      while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
        let operator: Token = self.previous();
        let right: Expr = self.term()?;
        expr = Expr::Binary { 
          left: Box::new(expr), 
          operator, 
          right: Box::new(right) 
        };
      }

      Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError>{
      let mut expr = self.comparison()?;

      while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
        let operator: Token = self.previous();
        let right: Expr = self.comparison()?;
        expr = Expr::Binary { 
          left: Box::new(expr), 
          operator, 
          right: Box::new(right) 
        };
      }

      Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
      self.equality()
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
      match self.expression() {
          Ok(expr) => Ok(expr),
          Err(err) => {
              self.synchronize(); 
              Err(err)
          }
      }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
      for token_type in types {
        if self.check(token_type.clone()) {
          self.advance();
          return true
        }
      }

      false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        self.peek().token_type == type_
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        if self.current == 0 {
          return self.tokens[0].clone();
        }
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn error(&self, token: Token, message: &str) -> ParserError {
      ParserError { token, message: message.to_string(), }
    }

    fn consume(&mut self, type_: TokenType, message: &str) -> Result<Token, ParserError> {
      if self.check(type_) {
          Ok(self.advance())
      } else {
          let token = self.peek();
          Err(self.error(token, message))
      }

    }

    fn synchronize(&mut self) {
      self.advance();

      while !self.is_at_end() {
        if self.previous().token_type == TokenType::Semicolon {
          return;
        }

        match self.peek().token_type {
            TokenType::Import 
          | TokenType::Export
          | TokenType::Match
          | TokenType::If
          | TokenType::Enum
          | TokenType::Type
          | TokenType::Trait
          | TokenType::Else
          | TokenType::While
          | TokenType::For
          | TokenType::Loop
          | TokenType::Fn
          | TokenType::Let
          | TokenType::Const
          | TokenType::Struct
          | TokenType::Component
          | TokenType::Server
          | TokenType::Client => return,
          _ => {}
        };

        self.advance();

      }
    }

}