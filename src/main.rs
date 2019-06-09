use std::fs;
use std::path::Path;

use inkwell::targets::InitializationConfig;
use inkwell::targets::Target;
use structopt::StructOpt;

pub mod decl;
pub mod expr;
pub mod stmt;

mod cli;
mod compiler;
mod lexer;
mod parser;

use cli::Opt;
use compiler::compile;
use lexer::lex;
use parser::parse;

fn main() {
    let opt = Opt::from_args();
    println!("file: {:?}", opt.file);

    let input = match fs::read_to_string(opt.file) {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    println!();
    println!("----- LEXER -----");
    println!();

    let tokens = lex(&input).unwrap();

    println!("tokens:");
    print!("  ");
    for token in &tokens {
        print!("{} ", token);
    }
    println!();

    assert!(!input.is_empty());

    println!();
    println!("----- PARSER ----");
    println!();

    let main = match parse(&tokens) {
        Ok(entry) => entry,
        Err(err) => panic!("{}", err),
    };

    println!("main:");
    println!("- {}", main);
    println!("- funcs:");
    for (name, func) in &main.funcs {
        println!("  - {}:", name.inner());
        println!("    - {}", func);
        println!("    - stmts:");
        for stmt in &func.stmts {
            println!("      - {}", stmt);
        }
    }

    // TODO: verifier

    println!();
    println!("---- COMPILER ---");
    println!();

    Target::initialize_x86(&InitializationConfig::default());

    let mut compiled = compile(&main).unwrap();
    println!("{}", compiled);

    compiled.create_target_machine();
    compiled.write_to_file(&Path::new("out.o")).unwrap();
}
