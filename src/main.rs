use std::fs;
use std::path::Path;

use inkwell::targets::InitializationConfig;
use inkwell::targets::Target;
use structopt::StructOpt;

mod ast;
mod cli;
mod compiler;
mod dependencies;
mod lexer;
mod parser;

use cli::Opt;
use compiler::compile;
use lexer::lex;
use lexer::Ident;
use parser::parse;

fn main() {
    let opt = Opt::from_args();
    println!("file: {:?}", opt.file);

    let input = match fs::read_to_string(opt.file) {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    println!();
    println!("-----------------");
    println!("------ LEX ------");
    println!("----- ( 1 ) -----");
    println!("-----------------");
    println!();

    let tokens = lex(&input).unwrap();

    println!("- tokens:  ");
    for token in &tokens {
        println!("  {} ", token);
    }
    println!();

    assert!(!input.is_empty());

    println!();
    println!("-----------------");
    println!("----- PARSE -----");
    println!("----- ( 2 ) -----");
    println!("-----------------");
    println!();

    let main = match parse(&tokens) {
        Ok(entry) => entry,
        Err(err) => panic!("{}", err),
    };

    println!("- modules:");
    println!("  {}:", main);
    println!("  - funcs:");
    for func in &main.funcs {
        println!("    {}:", func);
        println!("    - stmts:");
        for stmt in &func.stmts {
            println!("        {}", stmt);
        }
    }

    println!();
    println!("-----------------");
    println!("---- ANALYZE ----");
    println!("----- ( 3 ) -----");
    println!("-----------------");
    println!();

    println!();
    println!("-----------------");
    println!("------ AST ------");
    println!("---- ( 3.1 ) ----");
    println!("-----------------");
    println!();

    let mut ast = ast::Tree::build(&[&main]);
    let puts = Ident::Owned("puts".into());

    // --- FIXME ---
    ast.funcs.insert(
        puts.as_ref(),
        ast::Func {
            name: puts.as_ref(),
            ret: lexer::Ty::Void,
            start: None,
        },
    );

    for module in ast.modules.values_mut() {
        module.funcs.insert(puts.as_ref());
    }
    // --- FIXME ---

    println!("{}", ast);

    println!();
    println!("-----------------");
    println!("-- DEPENDENCIES -");
    println!("---- ( 3.2 ) ----");
    println!("-----------------");
    println!();

    let dependencies = dependencies::Graph::build(&ast);
    println!("{}", dependencies);

    println!();
    println!("-----------------");
    println!("----- VERIFY ----");
    println!("---- ( 3.2 ) ----");
    println!("-----------------");
    println!();

    dependencies.verify().unwrap();
    ast.verify().unwrap();

    // --- FIXME ---
    for module in ast.modules.values_mut() {
        module.funcs.remove(&puts);
    }

    ast.funcs.remove(&puts);
    // --- FIXME ---

    println!();
    println!("-----------------");
    println!("---- COMPILE ----");
    println!("----- ( 4 ) -----");
    println!("-----------------");
    println!();

    Target::initialize_x86(&InitializationConfig::default());

    let mut compiled = compile(&ast).unwrap();
    println!("{}", compiled);

    compiled.create_target_machine();
    compiled.write_to_file(&Path::new("out.o")).unwrap();
}
