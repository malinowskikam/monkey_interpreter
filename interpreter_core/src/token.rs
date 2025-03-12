#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Control
    ILLEGAL,
    EOF,

    // Identifiers/Literals
    IDENT,
    INT,

    //Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,

    //DELIMITERS
    COMMA,
    SEMICOLON,

    //BLOCKS
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,

    //KEYWORDS
    FUNCTION,
    LET,
    RETURN,
    TRUE,
    FALSE,
    IF,
    ELSE,
}

const LIT_FN: &[char] = &['f', 'n'];
const LIT_LET: &[char] = &['l', 'e', 't'];
const LIT_RETURN: &[char] = &['r', 'e', 't', 'u', 'r', 'n'];
const LIT_TRUE: &[char] = &['t', 'r', 'u', 'e'];
const LIT_FALSE: &[char] = &['f', 'a', 'l', 's', 'e'];
const LIT_IF: &[char] = &['i', 'f'];
const LIT_ELSE: &[char] = &['e', 'l', 's', 'e'];

impl From<&[char]> for TokenType {
    fn from(vec: &[char]) -> Self {
        match vec {
            LIT_FN => TokenType::FUNCTION,
            LIT_LET => TokenType::LET,
            LIT_RETURN => TokenType::RETURN,
            LIT_TRUE => TokenType::TRUE,
            LIT_FALSE => TokenType::FALSE,
            LIT_IF => TokenType::IF,
            LIT_ELSE => TokenType::ELSE,
            _ => TokenType::IDENT,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type() {
        let literals = ["abc", "fn", "let", "return", "true", "false", "if", "else"];

        let expected = [
            TokenType::IDENT,
            TokenType::FUNCTION,
            TokenType::LET,
            TokenType::RETURN,
            TokenType::TRUE,
            TokenType::FALSE,
            TokenType::IF,
            TokenType::ELSE,
        ];

        assert!(
            literals
                .iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .zip(expected.iter())
                .all(|(l, e)| TokenType::from(&l[..]) == *e)
        )
    }
}
