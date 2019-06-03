use std::fs;

mod compiler;
mod lexer;
mod parser;

use compiler::compile;
use lexer::lex;
use parser::parse;

fn main() {
    let input = fs::read_to_string("examples/empty_main.kt").unwrap();

    let (input, tokens) = lex(&input).unwrap();

    println!("rest: {:?}", input);
    for token in &tokens {
        print!("{} ", token);
    }
    println!();

    let entry = parse(&tokens).unwrap();
    println!("{}", entry);

    let compiled = compile(&entry).unwrap();
    let compiled = compiled.emit().unwrap();
    fs::write("out.o", compiled).unwrap();
}
