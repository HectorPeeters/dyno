mod lexer;

fn main() {
    let tokens = lexer::lex("12 + 9 / 4");
    for token in tokens {
        println!("{:?}", token);
    }
}
