package lexer

import (
	"testing"

	"github.com/malinowskikam/monkey_interpreter/internal/interpreter/token"
)

func TestNextToken(t *testing.T) {
	input := "=+(){},;"

	tests := []struct {
		ExpectedType    token.TokenType
		ExpectedLiteral string
	}{
		{token.EQUALS, "="},
		{token.PLUS, "+"},
		{token.LPAREN, "("},
		{token.RPAREN, ")"},
		{token.LBRACKET, "{"},
		{token.RBRACKET, "}"},
		{token.COMMA, ","},
		{token.SEMICOLON, ";"},
	}

	l := New(input)

	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.ExpectedType {
			t.Fatalf("test %d - tokentype wrong, expected: %s actual: %s", i, tt.ExpectedType, tok.Type)
		}

		if tok.Literal != tt.ExpectedLiteral {
			t.Fatalf("test %d - literal wrong, expected: %s actual: %s", i, tt.ExpectedLiteral, tok.Literal)
		}
	}
}

func TestNextToken2(t *testing.T) {
	input := `let five = 5;
let ten = 10;

let add = fn (x, y) {
	x + y;
};

let result = add(five, ten);
`

	tests := []struct {
		ExpectedType    token.TokenType
		ExpectedLiteral string
	}{
		{token.LET, "let"},
		{token.IDENT, "five"},
		{token.EQUALS, "="},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},

		{token.LET, "let"},
		{token.IDENT, "ten"},
		{token.EQUALS, "="},
		{token.INT, "10"},
		{token.SEMICOLON, ";"},

		{token.LET, "let"},
		{token.IDENT, "add"},
		{token.EQUALS, "="},
		{token.FUNCTION, "fn"},
		{token.LPAREN, "("},
		{token.IDENT, "x"},
		{token.COMMA, ","},
		{token.IDENT, "y"},
		{token.RPAREN, ")"},
		{token.LBRACKET, "{"},

		{token.IDENT, "x"},
		{token.PLUS, "+"},
		{token.IDENT, "y"},
		{token.SEMICOLON, ";"},

		{token.RBRACKET, "}"},
		{token.SEMICOLON, ";"},

		{token.LET, "let"},
		{token.IDENT, "result"},
		{token.EQUALS, "="},
		{token.IDENT, "add"},
		{token.LPAREN, "("},
		{token.IDENT, "five"},
		{token.COMMA, ","},
		{token.IDENT, "ten"},
		{token.RPAREN, ")"},
		{token.SEMICOLON, ";"},

		{token.EOF, ""},
	}

	l := New(input)

	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Type != tt.ExpectedType {
			t.Fatalf("test %d - tokentype wrong, expected: %s actual: %s", i, tt.ExpectedType, tok.Type)
		}

		if tok.Literal != tt.ExpectedLiteral {
			t.Fatalf("test %d - literal wrong, expected: %s actual: %s", i, tt.ExpectedLiteral, tok.Literal)
		}
	}
}
