#![allow(warnings)]

mod lexer;
mod parser;

use lexer::{tokenize, Token};
use parser::Parser;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    print!("\nIngresa la expresión a analizar: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Unable to read string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    print!("\n");

    let tokens = tokenize(&input);
    
    lexer::print_tokens(&tokens);

    let mut parser = Parser::new(&tokens);
    let expr = parser.parse_expression();
    println!("\nAST:\n{:?}", expr);
}