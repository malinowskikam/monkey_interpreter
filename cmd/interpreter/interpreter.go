package main

import (
	"fmt"

	"github.com/malinowskikam/monkey_interpreter/internal/interpreter/lexer"
	"github.com/malinowskikam/monkey_interpreter/internal/interpreter/token"
)

func main() {
	input := `
let five = 5;
let ten = 10;

let add = fn (x, y) {
	x + y;
};

let result = add(five, ten);
`
	l := lexer.New(input)
	t := l.NextToken()

	for t.Type != token.EOF {
		fmt.Printf("%v %v\n", t.Type, t.Literal)
		t = l.NextToken()
	}
}
