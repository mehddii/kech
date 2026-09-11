use lexer::*;

mod lexer;

fn main() {
    let program = "fct f() {}";

    let mut lexer = Lexer::new();
    lexer.feed(program);

    while !lexer.is_eof() {
        println!("{:?}", lexer.current());
        lexer.advance();
    }
}
