package lexer

import (
	"github.com/malinowskikam/monkey_interpreter/internal/interpreter/token"
)

type Lexer struct {
	input        string
	position     int
	readPosition int
	ch           byte
}

func New(input string) *Lexer {
	l := &Lexer{input: input}
	l.readChar()
	return l
}

func (l *Lexer) NextToken() token.Token {
	var tok token.Token
	l.skipWhitespace()

	switch l.ch {
	case '=':
		tok = token.Token{Type: token.EQUALS, Literal: string(l.ch)}
	case '+':
		tok = token.Token{Type: token.PLUS, Literal: string(l.ch)}
	case '(':
		tok = token.Token{Type: token.LPAREN, Literal: string(l.ch)}
	case ')':
		tok = token.Token{Type: token.RPAREN, Literal: string(l.ch)}
	case '{':
		tok = token.Token{Type: token.LBRACKET, Literal: string(l.ch)}
	case '}':
		tok = token.Token{Type: token.RBRACKET, Literal: string(l.ch)}
	case ',':
		tok = token.Token{Type: token.COMMA, Literal: string(l.ch)}
	case ';':
		tok = token.Token{Type: token.SEMICOLON, Literal: string(l.ch)}
	case 0:
		tok = token.Token{Type: token.EOF, Literal: ""}
	default:
		if isLetter(l.ch) {
			literal := l.readIdentifier()
			return token.Token{Type: token.LookupIdent(literal), Literal: literal}
		} else if isDigit(l.ch) {
			literal := l.readNumber()
			return token.Token{Type: token.INT, Literal: literal}
		} else {
			tok = token.Token{Type: token.ILLEGAL, Literal: string(l.ch)}
		}

	}

	l.readChar()

	return tok
}

func (l *Lexer) readChar() {
	if l.readPosition < len(l.input) {
		l.ch = l.input[l.readPosition]
	} else {
		l.ch = 0
	}

	l.position = l.readPosition
	l.readPosition += 1
}

func (l *Lexer) readIdentifier() string {
	position := l.position
	for isLetter(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func (l *Lexer) skipWhitespace() {
	for l.ch == ' ' || l.ch == '\t' || l.ch == '\r' || l.ch == '\n' {
		l.readChar()
	}
}

func isLetter(c byte) bool {
	return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

func isDigit(c byte) bool {
	return c >= '0' && c <= '9'
}
