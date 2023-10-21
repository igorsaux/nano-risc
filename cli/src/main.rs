use std::{fs, path::PathBuf};

use clap::Parser;
use nano_risc_arch::{Limits, SourceUnit};
use nano_risc_asm::{compiler, parser};
use nano_risc_vm::{VMStatus, VM};

#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// Path to an assembly file
    pub assembly: PathBuf,
}

fn main() {
    let app = Args::parse();
    let unit = SourceUnit::new(
        app.assembly.display().to_string(),
        fs::read(app.assembly).unwrap(),
    );
    let tokens = parser::parse(&unit).unwrap();
    let assembly = compiler::compile(unit, tokens, &Limits::default()).unwrap();
    let mut vm = VM::default();

    vm.set_dbg_callback(Box::new(|message| println!("{message}")));
    vm.load_assembly(assembly).unwrap();

    loop {
        match vm.tick() {
            Ok(VMStatus::Finished | VMStatus::Idle) => break,
            Err(error) => {
                eprintln!("Exception raised: {}", error.message());
                return;
            }
            _ => {}
        }
    }
}
