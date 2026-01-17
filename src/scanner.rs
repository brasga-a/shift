use crate::{errors, token::{Token, TokenType}};

#[derive(Debug, Clone)]
pub struct Scanner {
  source: Vec<char>,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  pub fn new(source: &str) -> Self {
    Self { 
      source: source.chars().collect(),
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
          self.start = self.current;
          self.scan_token();
        }

      self.tokens.push(Token::new(
        TokenType::Eof,
        String::new(),
        self.line
      ));

      self.tokens.clone()
  }

  //lexes
  fn scan_token(&mut self) {
    let c = self.advance();
    
    match c {
        '(' => self.add_token(TokenType::LeftParen),
        ')' => self.add_token(TokenType::RightParen),
        '{' => self.add_token(TokenType::LeftBrace),
        '}' => self.add_token(TokenType::RightBrace),
        '[' => self.add_token(TokenType::LeftBracket),
        ']' => self.add_token(TokenType::RightBracket),
        ',' => self.add_token(TokenType::Comma),
        '?' => { self.add_token(TokenType::Question)},
        ':' => self.add_token(TokenType::Colon),
        ';' => self.add_token(TokenType::Semicolon),
        '*' => self.add_token(TokenType::Star),
        '.' => { if self.match_char('.') { if self.match_char('.') { self.add_token(TokenType::Spread); } else { self.add_token(TokenType::Range); }} else { self.add_token(TokenType::Dot)}},
        '+' => { if self.match_char('+') { self.add_token(TokenType::Increment); } else { self.add_token(TokenType::Plus); }},
        '-' => { if self.match_char('-') { self.add_token(TokenType::Decrement);} else if self.match_char('>') { self.add_token(TokenType::SimpleArrow); } else { self.add_token(TokenType::Minus); } },
        '!' => { if self.match_char('=') { self.add_token(TokenType::BangEqual); } else { self.add_token(TokenType::Bang); }},
        '=' => { if self.match_char('=') { self.add_token(TokenType::EqualEqual); } else if self.match_char('>') { self.add_token(TokenType::Arrow); } else { self.add_token(TokenType::Equal); }},
        '<' => { if self.match_char('=') { self.add_token(TokenType::LessEqual); } else { self.add_token(TokenType::Less); }},
        '>' => { if self.match_char('=') { self.add_token(TokenType::GreaterEqual); } else { self.add_token(TokenType::Greater); }},
        '/' => { if self.match_char('/') { while self.peek() != '\n' && !self.is_at_end() { self.advance(); } } else if self.match_char('*') { self.scan_multiline_comment(); } else { self.add_token(TokenType::Slash); }},
        '&' => { if self.match_char('&') { self.add_token(TokenType::And);} else { self.add_token(TokenType::BitAnd); }},
        '|' => { if self.match_char('|') { self.add_token(TokenType::OrLogical);} else { self.add_token(TokenType::OrBitwise); }},
        
        ' ' | '\r' | '\t' => {},
        '\n' => { self.line += 1; },
        '"' => { self.string(); },
        _ => {
          if c.is_ascii_digit() { 
            self.number();
          } else if c.is_alphabetic() || c == '_' {
            self.identifier();
          } else { 
            errors::error(self.line, "Unexpected character.") 
          }
        },  
    }
  }

  fn scan_multiline_comment(&mut self) {
    let mut nest_level = 1;
    
    while nest_level > 0 && !self.is_at_end() {
        if self.peek() == '/' && self.peek_next() == '*' {
            // Achou /* aninhado
            self.advance(); self.advance();
            nest_level += 1;
        } else if self.peek() == '*' && self.peek_next() == '/' {
            // Achou */ fechando
            self.advance(); self.advance();
            nest_level -= 1;
        } else {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    }
    
    if nest_level > 0 {
    }
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn advance(&mut self) -> char {
    let c = self.source[self.current];
    self.current += 1;
    c
  }

  fn match_char(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false
    }
    if self.source[self.current] != expected {
      return false
    }

    self.current += 1;
    return true
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      self.source[self.current]
    }
  }

  fn identifier(&mut self) {
    while self.peek().is_alphanumeric() || self.peek() == '_' {
      self.advance();
    }

    let text: String = self.source[self.start..self.current].iter().collect();

    let token_type = match text.as_str() {
      
      // Rust
      "fn" | "function" => TokenType::Fn,
      "if"             => TokenType::If,
      "else"           => TokenType::Else,
      "return"         => TokenType::Return,
      "true"           => TokenType::True,
      "false"          => TokenType::False,
      "let"            => TokenType::Let,
      "const"          => TokenType::Const,
      "loop"           => TokenType::Loop,
      "while"          => TokenType::While,
      "for"            => TokenType::For,
      "enum"           => TokenType::Enum,
      "type"           => TokenType::Type,
      "trait"          => TokenType::Trait, 
      "match"          => TokenType::Match,

      // Shift
      "server"    => TokenType::Server,
      "client"    => TokenType::Client,
      "component" => TokenType::Component,
      "signal"    => TokenType::Signal,
      "derived"   => TokenType::Derived, 
      "effect"    => TokenType::Effect, 
      "prop"      => TokenType::Prop,
      "import"    => TokenType::Import,
      "export"    => TokenType::Export,
      "from"      => TokenType::From,
      "as"        => TokenType::As,

      _ => TokenType::Identifier(text)
    };

    self.add_token(token_type);
  }

  fn string (&mut self) {
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' { self.line += 1; }
      self.advance();
    }

    if self.is_at_end() {
      errors::error(self.line, "Unterminated string.");
      return;
    }

    self.advance();

    let text: String = self.source[(self.start + 1)..self.current]
        .iter()
        .collect();
    self.add_token(TokenType::String(text));
  }

  fn number(&mut self) {
    while self.peek().is_ascii_digit() {
      self.advance();
    }

    if self.peek() == '.' && self.peek_next().is_ascii_digit() {
      self.advance();

      while self.peek().is_ascii_digit() {
        self.advance();
      }
    }

   
    let text: String = self.source[self.start..self.current].iter().collect();

    if text.contains('.') {
      let value = text.parse::<f64>().unwrap_or(0.0);
      self.add_token(TokenType::Float(value));
    } else {
      let value = text.parse::<i64>().unwrap_or(0);
      self.add_token(TokenType::Integer(value));
    }
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0'
    }
    self.source[self.current + 1]
  }

  fn add_token(&mut self, token_type: TokenType) {
    let text: String = self.source[self.start..self.current]
        .iter()
        .collect();  
    
    self.tokens.push(Token::new(
        token_type,
        text,
        self.line
    ));
  }

}







