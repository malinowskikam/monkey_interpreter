use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
    done: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            done: false,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position < self.input.len() {
            self.ch = self.input[self.read_position];
        } else {
            self.ch = '\0';
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> &[char] {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char()
        }

        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &[char] {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char()
        }

        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char()
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        if self.done {
            return None;
        }

        let token = match self.ch {
            '=' => Token {
                token_type: TokenType::ASSIGN,
                literal: self.ch.into(),
            },
            '+' => Token {
                token_type: TokenType::PLUS,
                literal: self.ch.into(),
            },
            '-' => Token {
                token_type: TokenType::MINUS,
                literal: self.ch.into(),
            },
            '!' => Token {
                token_type: TokenType::BANG,
                literal: self.ch.into(),
            },
            '/' => Token {
                token_type: TokenType::SLASH,
                literal: self.ch.into(),
            },
            '*' => Token {
                token_type: TokenType::ASTERISK,
                literal: self.ch.into(),
            },
            '<' => Token {
                token_type: TokenType::LT,
                literal: self.ch.into(),
            },
            '>' => Token {
                token_type: TokenType::GT,
                literal: self.ch.into(),
            },
            '(' => Token {
                token_type: TokenType::LPAREN,
                literal: self.ch.into(),
            },
            ')' => Token {
                token_type: TokenType::RPAREN,
                literal: self.ch.into(),
            },
            '{' => Token {
                token_type: TokenType::LBRACKET,
                literal: self.ch.into(),
            },
            '}' => Token {
                token_type: TokenType::RBRACKET,
                literal: self.ch.into(),
            },
            ',' => Token {
                token_type: TokenType::COMMA,
                literal: self.ch.into(),
            },
            ';' => Token {
                token_type: TokenType::SEMICOLON,
                literal: self.ch.into(),
            },
            '\0' => {
                self.done = true;
                Token {
                    token_type: TokenType::EOF,
                    literal: "".into(),
                }
            }
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    return Some(Token {
                        token_type: literal.into(),
                        literal: literal.iter().collect(),
                    });
                } else if self.ch.is_numeric() {
                    let literal = self.read_number();
                    return Some(Token {
                        token_type: TokenType::INT,
                        literal: literal.iter().collect(),
                    });
                } else {
                    Token {
                        token_type: TokenType::ILLEGAL,
                        literal: self.ch.into(),
                    }
                }
            }
        };

        self.read_char();

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_simple() {
        let input = "1 + 2;";

        let lexer = Lexer::new(input);

        let expected = &[
            Token {
                token_type: TokenType::INT,
                literal: "1".into(),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: "+".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "2".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },
            Token {
                token_type: TokenType::EOF,
                literal: "".into(),
            },
        ];

        assert_eq!(lexer.collect::<Vec<Token>>(), expected);
    }

    #[test]
    fn test_lexer_big() {
        let input = "let five = 5;
let ten = 10;

let add = fn (x, y) {
	x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}";

        let lexer = Lexer::new(input);

        let expected = &[
            Token {
                token_type: TokenType::LET,
                literal: "let".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "five".into(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "ten".into(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "add".into(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".into(),
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: "fn".into(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "x".into(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "y".into(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".into(),
            },
            Token {
                token_type: TokenType::LBRACKET,
                literal: "{".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "x".into(),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: "+".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "y".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },
            Token {
                token_type: TokenType::RBRACKET,
                literal: "}".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },
            Token {
                token_type: TokenType::LET,
                literal: "let".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "result".into(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "add".into(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "five".into(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".into(),
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "ten".into(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },

            Token {
                token_type: TokenType::BANG,
                literal: "!".into(),
            },
            Token {
                token_type: TokenType::MINUS,
                literal: "-".into(),
            },
            Token {
                token_type: TokenType::SLASH,
                literal: "/".into(),
            },
            Token {
                token_type: TokenType::ASTERISK,
                literal: "*".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },

            Token {
                token_type: TokenType::INT,
                literal: "5".into(),
            },
            Token {
                token_type: TokenType::LT,
                literal: "<".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".into(),
            },
            Token {
                token_type: TokenType::GT,
                literal: ">".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },

            Token {
                token_type: TokenType::IF,
                literal: "if".into(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "5".into(),
            },
            Token {
                token_type: TokenType::LT,
                literal: "<".into(),
            },
            Token {
                token_type: TokenType::INT,
                literal: "10".into(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".into(),
            },
            Token {
                token_type: TokenType::LBRACKET,
                literal: "{".into(),
            },

            Token {
                token_type: TokenType::RETURN,
                literal: "return".into(),
            },
            Token {
                token_type: TokenType::TRUE,
                literal: "true".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },

            Token {
                token_type: TokenType::RBRACKET,
                literal: "}".into(),
            },
            Token {
                token_type: TokenType::ELSE,
                literal: "else".into(),
            },
            Token {
                token_type: TokenType::LBRACKET,
                literal: "{".into(),
            },

            Token {
                token_type: TokenType::RETURN,
                literal: "return".into(),
            },
            Token {
                token_type: TokenType::FALSE,
                literal: "false".into(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".into(),
            },

            Token {
                token_type: TokenType::RBRACKET,
                literal: "}".into(),
            },

            Token {
                token_type: TokenType::EOF,
                literal: "".into(),
            },
        ];

        assert_eq!(lexer.collect::<Vec<Token>>(), expected);
    }
}
