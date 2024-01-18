package main

import (
	"fmt"
	"os"

	"github.com/malinowskikam/monkey_interpreter/internal/interpreter/lexer"
	"github.com/malinowskikam/monkey_interpreter/internal/interpreter/token"
)

func main() {
	if len(os.Args) > 1 {
		switch os.Args[1] {
		case "lexer":
			cmdLexer()
		default:
			fmt.Printf("unknown command: %v\n", os.Args[1])
		}
	} else {
		fmt.Printf("usage: interpreter <command> [args]\n")
	}
}

func cmdLexer() {
	if len(os.Args) > 2 {
		input := os.Args[2]

		l := lexer.New(input)
		t := l.NextToken()

		for t.Type != token.EOF {
			fmt.Printf("%-12v %v\n", t.Type, t.Literal)
			t = l.NextToken()
		}
	} else {
		fmt.Printf("usage: interpreter lexer <input>\n")
	}
}
