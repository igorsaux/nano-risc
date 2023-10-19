use std::{fs, path::PathBuf};

use clap::Parser;
use vm::{Program, VMStatus, VM};

#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// Path to an assembly file
    pub assembly: PathBuf,
}

fn main() {
    let app = Args::parse();
    let assembly = parser::Parser::new_bytes(fs::read(app.assembly).unwrap())
        .parse()
        .unwrap();

    println!("Compiling program...");

    let program = Program::try_compile(assembly).unwrap();
    let mut vm = VM::default();

    println!("Loading program...");

    vm.load_program(program);

    println!("Executing...");

    while let VMStatus::Running | VMStatus::Yield = vm.tick().unwrap() {}
}
