use interpreter_core::lexer::Lexer;

fn main() {
    let s = "let five = 5;
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

    let lexer = Lexer::new(s);

    for token in lexer {
        println!("{:?}", token);
    }
}
