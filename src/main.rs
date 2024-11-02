mod lexer;
mod parser;

use lexer::{tokenize, Token};
use parser::Parser;

fn main() {
    let input = "sin(0.5)^2 + cos(y) - tan(1.0)";
    
    let tokens = tokenize(input);
    
    lexer::print_tokens(&tokens);

    let mut parser = Parser::new(&tokens);
    let expr = parser.parse_expression();
    println!("\nArbol de sintaxis abstracta:\n{:?}", expr);
}

