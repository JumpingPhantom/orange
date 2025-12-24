use orange::{lexer::Lexer, parser::Parser};

fn main() {
    let mut _lexer = Lexer::new("samples/nocap.ong").tokenize();
    let mut _parser = Parser::new(_lexer.tokens).parse();
}
