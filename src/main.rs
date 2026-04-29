mod lexer;
mod parser;
mod ast;
mod semantic;
mod ir;
mod optimizer;
mod codegen;

use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use clap::Parser as ClapParser;

#[derive(ClapParser)]
#[command(name = "cc-compiler")]
#[command(about = "Compiler for a graphics DSL to x86 assembly", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short = 'o', long, default_value = "output.asm")]
    output: PathBuf,

    #[arg(short = 'O', long, action = clap::ArgAction::SetTrue)]  // changed from -o to -O
    optimize: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let source = fs::read_to_string(&cli.input)?;

    // 1. Parse
    let ast = parser::parse_program(&source)?;

    // 2. Semantic analysis
    semantic::semantic_check(&ast)?;

    // 3. Generate IR
    let mut ir_gen = ir::IRGenerator::new();
    ir_gen.generate(&ast)?;
    let mut ir_code = ir_gen.code;

    // 4. Optimize IR (optional)
    if cli.optimize {
        ir_code = optimizer::constant_fold(ir_code);
    }

    // 5. Code generation
    let mut asm_gen = codegen::X86Generator::new();
    asm_gen.generate(ir_code, cli.output.to_str().unwrap())?;

    println!("Compilation successful!");
    Ok(())
}