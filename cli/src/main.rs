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

    let program = Program::try_compile(assembly).unwrap();
    let mut vm = VM::default();

    vm.set_dbg_callback(Box::new(|message| println!("{message}")));
    vm.load_program(program);

    loop {
        match vm.tick() {
            Ok(VMStatus::Finished | VMStatus::Idle) => break,
            Err(error) => {
                eprintln!("Exception raised: {error}");
                return;
            }
            _ => {}
        }
    }
}
