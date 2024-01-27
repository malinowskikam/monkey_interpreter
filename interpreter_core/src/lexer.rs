use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Iterator for Lexer {
    type Item = Token<>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token{token_type: TokenType::ASSIGN, literal: [self.ch]},
            '+' => Token{token_type: TokenType::PLUS, literal: self.ch.into()},
            '-' => Token{token_type: TokenType::MINUS, literal: self.ch.into()},
            '!' => Token{token_type: TokenType::BANG, literal: self.ch.into()},
            '/' => Token{token_type: TokenType::SLASH, literal: self.ch.into()},
            '*' => Token{token_type: TokenType::ASTERISK, literal: self.ch.into()},
            '<' => Token{token_type: TokenType::LT, literal: self.ch.into()},
            '>' => Token{token_type: TokenType::GT, literal: self.ch.into()},
            '(' => Token{token_type: TokenType::LPAREN, literal: self.ch.into()},
            ')' => Token{token_type: TokenType::RPAREN, literal: self.ch.into()},
            '{' => Token{token_type: TokenType::LBRACKET, literal: self.ch.into()},
            '}' => Token{token_type: TokenType::RBRACKET, literal: self.ch.into()},
            ',' => Token{token_type: TokenType::COMMA, literal: self.ch.into()},
            ';' => Token{token_type: TokenType::SEMICOLON, literal: self.ch.into()},
            0 => Token{token_type: TokenType::EOF, literal: &[]},
            _ => {
                if is_letter(&self.ch) {
                    let literal = self.read_identifier();
                    return Some(Token {token_type: literal.into(), literal});
                } else if is_digit(&self.ch) {
                    let literal = self.read_number();
                    return Ok(Token {token_type: literal.into(), literal});
                } else {
                    Token{token_type: TokenType::ILLEGAL, literal: self.ch.into()}
                }
            }
        };

        self.read_char();

        Some(token)
    }
}

impl<'a> Lexer {
    fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().map(|c| &c).collect(),
            position: 0,
            read_position: 0,
            ch: 0.into(),
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position < self.input.len() {
            self.ch = *self.input[self.read_position]
        } else {
            self.ch = 0.into()
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &[char] {
        let position = self.position;
        while is_letter(&self.ch) {
            self.read_char()
        }
        return &self.input[position..self.position];
    }

    fn read_number(&mut self) -> &[&char] {
        let position = self.position;
        while is_digit(&self.ch) {
            self.read_char()
        }
        return &self.input[position..self.position];
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(&self.ch) {
            self.read_char()
        }
    }
}

fn is_whitespace(c: &char) -> bool {
    c.is_whitespace()
}

fn is_digit(c: &char) -> bool {
    c.is_ascii_digit()
}

fn is_letter(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == '_'
}
