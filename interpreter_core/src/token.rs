#[derive(Debug)]
pub enum TokenType {
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

    //BLOCS
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,

    //KEYWORDS
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl<'a> From<&[char]> for Token<'a> {
    fn from(literal: &[char]) -> Self {
        let token_type = match literal.iter().collect::<String>().as_str() {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        };
        Token {token_type, literal}
    }
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a[char],
}
