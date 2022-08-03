//input: one expresion
//output: tokens
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tokens {
  INVALID,
  EQ, // "="
  LEQ, //"<="
  LT, // "<"
  PLUS, // "+"
  MULT, // "*"
  IF, //"if"
  ELSE, //"else"
  THEN, //"then"
  FUNC, //"f"
  LET, //"let"
  IN, //"in"
  UNIT, // ":="
  SEMICOLON, // ";"
  INT, // integer
  STRING, // string
  ID, // variables
  LPAREN, // "("
  RPAREN, // ")"
  COMMA, // ","
}

#[derive(Debug, Error)]
pub enum LexerError {
  #[error("invalid tokens at position {0}")]
  InvalidToken(usize),
}

pub struct Lexer {
  input : Vec<char>,
  output: Vec<(Tokens, String)>,
  pos_start: usize,
  pos_current: usize,
  last_token: Tokens,
  last_pos: usize,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let lexer = Lexer {
      input: input.chars().collect(),
      output: Vec::new(),
      pos_start: 0,
      pos_current: 0,
      last_token: Tokens::INVALID,
      last_pos: 0,
    };
    lexer
  }
  pub fn lex(&mut self) -> Result<Vec<(Tokens, String)>, LexerError> {
    self.q0()?;
    Ok(self.output.clone())
  }
  fn get_lexerme(&self) -> String {
    self.input[self.pos_start..self.last_pos].iter().collect::<String>().clone()
  }
  fn output_token(&mut self) {
    let token = self.last_token;
    if token == Tokens::INVALID {
      return;
    }
    let item = self.get_lexerme();
    self.output.push((token, item));
  }
  fn readc(&mut self) -> char {
    if self.pos_current >= self.input.len() {
      return '\0';
    }
    let c = self.input[self.pos_current];
    self.pos_current += 1;
    c
  }
  fn save(&mut self, tokens: Tokens) {
    self.last_token = tokens;
    self.last_pos = self.pos_current;
  }
  fn q0(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      ' ' | '\t' | '\n' | '\r' => {
        self.pos_start += 1;
        self.q0()
      }
      '=' => {
        self.save(Tokens::EQ);
        self.next()
      }
      '<' => {
        self.save(Tokens::LT);
        self.q_lt()
      }
      '+' => {
        self.save(Tokens::PLUS);
        self.next()
      }
      '*' => {
        self.save(Tokens::MULT);
        self.next()
      }
      'i' => {
        self.save(Tokens::ID);
        self.q_i()
      }
      'e' => {
        self.save(Tokens::ID);
        self.q_e()
      }
      't' => {
        self.save(Tokens::ID);
        self.q_t()
      }
      'f' => {
        self.save(Tokens::ID);
        self.q_f()
      }
      'l' => {
        self.save(Tokens::ID);
        self.q_l()
      }
      ':' => {
        self.q_unit()
      }
      ';' => {
        self.save(Tokens::SEMICOLON);
        self.next()
      }
      '0' => {
        self.save(Tokens::INT);
        self.next()
      }
      '\"' => {
        self.q_string()
      }
      c => {
        if c.is_alphabetic() {
          self.save(Tokens::ID);
          self.q_sym()
        } else if c.is_numeric() {
          self.save(Tokens::INT);
          self.q_num()
        } else if c == '\0' {
          Ok(())
        } else {
          Err(LexerError::InvalidToken(self.pos_current))
        }
      }
    }
  }
  fn next(&mut self) -> Result<(), LexerError> {
    if self.last_token == Tokens::INVALID {
      Err(LexerError::InvalidToken(self.pos_current))
    } else {
      self.output_token();
      self.pos_start = self.last_pos;
      self.pos_current = self.pos_start;
      self.last_token = Tokens::INVALID;
      self.q0()
    }
  }
  fn fnext(&mut self) -> Result<(), LexerError> {
    if self.last_token == Tokens::INVALID {
      Err(LexerError::InvalidToken(self.pos_current))
    } else {
      self.output_token();
      self.pos_start = self.last_pos;
      self.pos_current = self.pos_start;
      self.last_token = Tokens::INVALID;
      self.q_flp()
    }
  }
  fn q_lt(&mut self) -> Result<(), LexerError> {
    if self.readc() == '=' {
      self.save(Tokens::LEQ);
      self.next()
    } else {
      self.next()
    }
  }
  fn q_i(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      'f' => {
        self.save(Tokens::IF);
        self.q_sym()
      }
      'n' => {
        self.save(Tokens::IN);
        self.q_sym()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_e(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      'l' => {
        self.save(Tokens::ID);
        self.q_el()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_t(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      'h' => {
        self.save(Tokens::ID);
        self.q_th()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_f(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      ' ' | '\t'  => {
        self.q_f()
      }
      '(' => {
        self.save(Tokens::FUNC);
        self.last_pos -= 1;
        self.output_token();
        self.pos_start += 1;
        self.last_pos += 1;
        self.save(Tokens::LPAREN);
        self.output_token();
        self.pos_start += 1;
        self.q_flp()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_el(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      's' => {
        self.save(Tokens::ID);
        self.q_els()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_unit(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      '=' => {
        self.save(Tokens::UNIT);
        self.next()
      }
      _ => {
        Err(LexerError::InvalidToken(self.pos_current))
      }
    }
  }
  fn q_sym(&mut self) -> Result<(), LexerError> {
    let c = self.readc();
    if c.is_alphabetic() || c.is_numeric() {
      self.save(Tokens::ID);
      self.q_sym()
    } else {
      self.next()
    }
  }
  fn q_fsym(&mut self) -> Result<(), LexerError> {
    let c = self.readc();
    if c.is_alphabetic() || c.is_numeric() {
      self.save(Tokens::ID);
      self.q_fsym()
    } else {
      self.fnext()
    }
  }
  fn q_num(&mut self) -> Result<(), LexerError> {
    let c = self.readc();
    if c.is_numeric() {
      self.save(Tokens::INT);
      self.q_num()
    } else {
      self.next()
    }
  }
  fn q_flp(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      ')' => {
        self.save(Tokens::RPAREN);
        self.next()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_fsym()
        } else if c == ' ' || c == '\t' {
          self.pos_start += 1;
          self.q_flp()
        } else if c == ',' {
          self.save(Tokens::COMMA);
          self.output_token();
          self.pos_start += 1;
          self.q_flp()
        } else {
          Err(LexerError::InvalidToken(self.pos_current))
        }
      }
    }
  }
  fn q_els(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      'e' => {
        self.save(Tokens::ELSE);
        self.q_t()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_th(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      'e' => {
        self.save(Tokens::ID);
        self.q_the()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_the(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      'n' => {
        self.save(Tokens::THEN);
        self.q_sym()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_l(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      'e' => {
        self.save(Tokens::ID);
        self.q_le()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_le(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      't' => {
        self.save(Tokens::LET);
        self.q_sym()
      }
      c => {
        if c.is_alphabetic() || c.is_numeric() {
          self.save(Tokens::ID);
          self.q_sym()
        } else {
          self.next()
        }
      }
    }
  }
  fn q_string(&mut self) -> Result<(), LexerError> {
    match self.readc() {
      '"' => {
        self.save(Tokens::STRING);
        self.next()
      }
      _ => {
        self.q_string()
      }
    }
  }
}
#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn lexer_test() {
    let mut input = Vec::new();
    input.push("let x = 1 in x");
    input.push("let x = 1 in x + 1");
    input.push("if x = 2 then y := 3 else y := x * 4");
    input.push("f(x, y, z) = x + y * z");
    input.push("x := 1; y := 2; z := 3");
    input.push("x := \"test\"");
    let mut expected = Vec::new();
    expected.push(vec![(Tokens::LET, "let".to_string()), (Tokens::ID, "x".to_string()), (Tokens::EQ, "=".to_string()), (Tokens::INT, "1".to_string()), (Tokens::IN, "in".to_string()), (Tokens::ID, "x".to_string())]);
    expected.push(vec![(Tokens::LET, "let".to_string()), (Tokens::ID, "x".to_string()), (Tokens::EQ, "=".to_string()), (Tokens::INT, "1".to_string()), (Tokens::IN, "in".to_string()), (Tokens::ID, "x".to_string()), (Tokens::PLUS, "+".to_string()), (Tokens::INT, "1".to_string())]);
    expected.push(vec![(Tokens::IF, "if".to_string()), (Tokens::ID, "x".to_string()), (Tokens::EQ, "=".to_string()), (Tokens::INT, "2".to_string()), (Tokens::THEN, "then".to_string()), (Tokens::ID, "y".to_string()), (Tokens::UNIT, ":=".to_string()), (Tokens::INT, "3".to_string()), (Tokens::ELSE, "else".to_string()), (Tokens::ID, "y".to_string()), (Tokens::UNIT, ":=".to_string()), (Tokens::ID, "x".to_string()), (Tokens::MULT, "*".to_string()), (Tokens::INT, "4".to_string())]);
    expected.push(vec![(Tokens::FUNC, "f".to_string()), (Tokens::LPAREN, "(".to_string()), (Tokens::ID, "x".to_string()), (Tokens::COMMA, ",".to_string()), (Tokens::ID, "y".to_string()), (Tokens::COMMA, ",".to_string()), (Tokens::ID, "z".to_string()), (Tokens::RPAREN, ")".to_string()), (Tokens::EQ, "=".to_string()), (Tokens::ID, "x".to_string()), (Tokens::PLUS, "+".to_string()), (Tokens::ID, "y".to_string()), (Tokens::MULT, "*".to_string()), (Tokens::ID, "z".to_string())]);
    expected.push(vec![(Tokens::ID, "x".to_string()), (Tokens::UNIT, ":=".to_string()), (Tokens::INT, "1".to_string()), (Tokens::SEMICOLON, ";".to_string()), (Tokens::ID, "y".to_string()), (Tokens::UNIT, ":=".to_string()), (Tokens::INT, "2".to_string()), (Tokens::SEMICOLON, ";".to_string()), (Tokens::ID, "z".to_string()), (Tokens::UNIT, ":=".to_string()), (Tokens::INT, "3".to_string())]);
    expected.push(vec![(Tokens::ID, "x".to_string()), (Tokens::UNIT, ":=".to_string()), (Tokens::STRING, "\"test\"".to_string())]);

    for i in 0..6 {
      let mut lexer = Lexer::new(input[i]);
      assert_eq!(lexer.lex().unwrap(), expected[i]);
    }
  }
}