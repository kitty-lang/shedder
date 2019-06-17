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
use lexer::Ty;
use parser::decl::Arg;
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

    // --- FIXME ---
    let printf = Ident::Owned("printf".into());
    let printf_args = vec![Arg {
        name: Ident::Owned("printf".into()),
        ty: Ty::Str,
    }];

    ast.funcs.insert(
        printf.as_ref(),
        ast::Func {
            name: printf.as_ref(),
            args: &printf_args,
            ret: lexer::Ty::Void,
            variadic: true,
            start: None,
        },
    );

    for module in ast.modules.values_mut() {
        module.funcs.insert(printf.as_ref());
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
    println!("DEPENDENCIES: OK");

    ast.verify().unwrap();
    println!("AST: OK");

    // --- FIXME ---
    for module in ast.modules.values_mut() {
        module.funcs.remove(&printf);
    }

    ast.funcs.remove(&printf);
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
