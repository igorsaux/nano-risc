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

#[cfg(test)]
mod tests {
    use parser::Parser;
    use vm::{Program, RuntimeError, VMStatus, Value};

    use crate::VM;

    fn create_vm_from(source: &str) -> VM {
        let assembly = Parser::new_string(source.to_string()).parse().unwrap();
        let program = Program::try_compile(assembly).unwrap();
        let mut vm = VM::default();

        vm.load_program(program);

        vm
    }

    #[test]
    fn add() {
        let source = r#"
            add r0 0 1
            add r1 0 1
            add r2 r0 r1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Finished));

        assert_eq!(vm.registers[0], Value::Float { value: 1.0 });
        assert_eq!(vm.registers[1], Value::Float { value: 1.0 });
        assert_eq!(vm.registers[2], Value::Float { value: 2.0 });
    }

    #[test]
    fn sub_mov() {
        let source = r#"
            mov 4 r0
            mov 2 r1
            sub r0 r0 r1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 4.0 });

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[1], Value::Float { value: 2.0 });

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 2.0 });

        assert_eq!(vm.tick(), Ok(VMStatus::Finished));
    }

    #[test]
    fn jump() {
        let source = r#"
            start:
                add r0 0 1

                jmp start
        "#;
        let mut vm = create_vm_from(source);
        assert_eq!(vm.pc, 0);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.pc, 1);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.pc, 0);
    }

    #[test]
    fn yld() {
        let source = r#"
            add r0 0 1

            yield
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Yield));
        assert_eq!(vm.tick(), Ok(VMStatus::Finished));
    }

    #[test]
    fn mul() {
        let source = r#"
            mul r0 5 2
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 10.0 })
    }

    #[test]
    fn div() {
        let source = r#"
            div r0 10 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 2.0 })
    }

    #[test]
    fn div_by_zero() {
        let source = r#"
            div r0 10 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Err(RuntimeError::DividedByZero));
    }

    #[test]
    fn r#mod() {
        let source = r#"
            mod r0 10 6
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 4.0 })
    }

    #[test]
    fn beq() {
        let source = r#"
            beq 0 0 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn bge() {
        let source = r#"
            bge 5 5 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn bgt() {
        let source = r#"
            bgt 10 5 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn ble() {
        let source = r#"
            ble 5 5 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn blt() {
        let source = r#"
            ble 5 10 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn bne() {
        let source = r#"
            bne 22 11 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn beqz() {
        let source = r#"
            beqz 0 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn bgez() {
        let source = r#"
            bgez 0 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn bgtz() {
        let source = r#"
            bgez 5 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn blez() {
        let source = r#"
            blez 0 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn bltz() {
        let source = r#"
            blez -10 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn bnez() {
        let source = r#"
            bnez 25 success
            yield

            success:
            mov 1 r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn seq() {
        let source = r#"
            seq r0 0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn sge() {
        let source = r#"
            sge r0 5 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn sgt() {
        let source = r#"
            sgt r0 10 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn sle() {
        let source = r#"
            sle r0 5 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn slt() {
        let source = r#"
            sle r0 5 10
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn sne() {
        let source = r#"
            sne r0 22 11
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn seqz() {
        let source = r#"
            seqz r0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn sgez() {
        let source = r#"
            sgez r0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn sgtz() {
        let source = r#"
            sgez r0 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn slez() {
        let source = r#"
            slez r0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn sltz() {
        let source = r#"
            slez r0 -10
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }

    #[test]
    fn snez() {
        let source = r#"
            snez r0 25
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers[0], Value::Float { value: 1.0 })
    }
}
