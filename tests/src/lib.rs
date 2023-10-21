#[cfg(test)]
mod vm_tests {
    use nano_risc_arch::SourceUnit;
    use nano_risc_asm::{compiler, parser};
    use nano_risc_vm::{RuntimeErrorKind, VMStatus, VM};

    fn create_vm_from(source: &str) -> VM {
        let unit = SourceUnit::new_anonymous(source.as_bytes().to_vec());
        let tokens = parser::parse(&unit).unwrap();
        let assembly = compiler::compile(unit, tokens, None).unwrap();
        let mut vm = VM::default();

        vm.load_assembly(assembly).unwrap();

        vm
    }

    #[test]
    fn add() {
        let source = r#"
            add $r0 0 1
            add $r1 0 1
            add $r2 $r0 $r1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Finished));

        assert_eq!(vm.registers()[0], 1.0);
        assert_eq!(vm.registers()[1], 1.0);
        assert_eq!(vm.registers()[2], 2.0);
    }

    #[test]
    fn sub_mov() {
        let source = r#"
            mov $r0 4
            mov $r1 2
            sub $r0 $r0 $r1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 4.0);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[1], 2.0);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 2.0);

        assert_eq!(vm.tick(), Ok(VMStatus::Finished));
    }

    #[test]
    fn jmp_label() {
        let source = r#"
            start:
                add $r0 0 1

                jmp start
        "#;
        let mut vm = create_vm_from(source);
        assert_eq!(vm.pc(), 0);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.pc(), 1);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.pc(), 0);
    }

    #[test]
    fn jmp_number() {
        let source = r#"
            start:
                add $r0 0 1

                jmp 0
        "#;
        let mut vm = create_vm_from(source);
        assert_eq!(vm.pc(), 0);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.pc(), 1);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.pc(), 0);
    }

    #[test]
    fn yld() {
        let source = r#"
            add $r0 0 1

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
            mul $r0 5 2
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 10.0)
    }

    #[test]
    fn div() {
        let source = r#"
            div $r0 10 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 2.0)
    }

    #[test]
    fn div_by_zero() {
        let source = r#"
            div $r0 10 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(
            vm.tick().map_err(|err| err.kind().clone()),
            Err(RuntimeErrorKind::DividedByZero)
        );
    }

    #[test]
    fn r#mod() {
        let source = r#"
            mod $r0 10 6
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 4.0)
    }

    #[test]
    fn beq() {
        let source = r#"
            beq 0 0 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn bge() {
        let source = r#"
            bge 5 5 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn bgt() {
        let source = r#"
            bgt 10 5 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn ble() {
        let source = r#"
            ble 5 5 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn blt() {
        let source = r#"
            ble 5 10 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn bne() {
        let source = r#"
            bne 22 11 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn beqz() {
        let source = r#"
            beqz 0 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn bgez() {
        let source = r#"
            bgez 0 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn bgtz() {
        let source = r#"
            bgez 5 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn blez() {
        let source = r#"
            blez 0 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn bltz() {
        let source = r#"
            blez -10 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn bnez() {
        let source = r#"
            bnez 25 success
            yield

            success:
            mov $r0 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn seq() {
        let source = r#"
            seq $r0 0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn sge() {
        let source = r#"
            sge $r0 5 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn sgt() {
        let source = r#"
            sgt $r0 10 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn sle() {
        let source = r#"
            sle $r0 5 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn slt() {
        let source = r#"
            sle $r0 5 10
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn sne() {
        let source = r#"
            sne $r0 22 11
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn seqz() {
        let source = r#"
            seqz $r0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn sgez() {
        let source = r#"
            sgez $r0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn sgtz() {
        let source = r#"
            sgez $r0 5
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn slez() {
        let source = r#"
            slez $r0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn sltz() {
        let source = r#"
            slez $r0 -10
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn snez() {
        let source = r#"
            snez $r0 25
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.registers()[0], 1.0)
    }

    #[test]
    fn halt() {
        let source = r#"
            mov $r1 1
            halt
            mov $r0 0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Finished))
    }

    #[test]
    fn invalid_jmp() {
        let source = r#"
            jmp 9999
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(
            vm.tick().map_err(|err| err.kind().clone()),
            Err(RuntimeErrorKind::InvalidAddress { address: 9999 })
        )
    }

    #[test]
    fn push() {
        let source = r#"
            push 1
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.sp(), 1);
        assert_eq!(vm.stack()[0], 1.0)
    }

    #[test]
    fn pop() {
        let source = r#"
            push 1
            pop $r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.sp(), 0);
        assert_eq!(vm.stack()[0], 0.0);
        assert_eq!(vm.registers()[0], 1.0);
    }

    #[test]
    fn peek() {
        let source = r#"
            push 1
            peek $r0
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.sp(), 1);
        assert_eq!(vm.stack()[0], 1.0);
        assert_eq!(vm.registers()[0], 1.0);
    }

    #[test]
    fn call_ret() {
        let source = r#"
            main:
                call sum
                dbg $r0
                halt

            sum:
                add $r0 2 2
                ret
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));

        assert_eq!(vm.stack()[0], 1.0);
        assert_eq!(vm.pc(), 3);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));

        assert_eq!(vm.sp(), 0);
        assert_eq!(vm.pc(), 1);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Finished));
    }

    #[test]
    fn indirect_registers() {
        let source = r#"
            mov $r0 1
            mov $r1 100

            mov $r2 %r0
            beq $r2 100 success
            halt

            success:
                mov $r0 100
        "#;
        let mut vm = create_vm_from(source);

        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));
        assert_eq!(vm.tick(), Ok(VMStatus::Running));

        assert_eq!(vm.registers()[0], 100.0);
    }

    #[test]
    fn logical() {
        let source = r#"
            and $r0 0 1
            or $r1 0 1
            xor $r2 0 1
            nor $r3 0 1
        "#;
        let mut vm = create_vm_from(source);

        while let VMStatus::Running = vm.tick().unwrap() {}

        assert_eq!(vm.registers()[0], 0.0);
        assert_eq!(vm.registers()[1], 1.0);
        assert_eq!(vm.registers()[2], 1.0);
        assert_eq!(vm.registers()[3], 0.0);
    }

    #[test]
    fn bitwise() {
        let source = r#"
            andi $r0 2 3
            ori $r1 2 3
            xori $r2 2 3
        "#;
        let mut vm = create_vm_from(source);

        while let VMStatus::Running = vm.tick().unwrap() {}

        assert_eq!(vm.registers()[0], (2 & 3) as f32);
        assert_eq!(vm.registers()[1], (2 | 3) as f32);
        assert_eq!(vm.registers()[2], (2 ^ 3) as f32);
    }

    #[test]
    fn shifts() {
        let source = r#"
            shr $r0 1 5
            shl $r1 1 5
            ror $r2 1 5
            rol $r3 1 5
        "#;
        let mut vm = create_vm_from(source);

        while let VMStatus::Running = vm.tick().unwrap() {}

        assert_eq!(vm.registers()[0], i32::wrapping_shr(1, 5) as f32);
        assert_eq!(vm.registers()[1], i32::wrapping_shl(1, 5) as f32);
        assert_eq!(vm.registers()[2], i32::rotate_right(1, 5) as f32);
        assert_eq!(vm.registers()[3], i32::rotate_left(1, 5) as f32);
    }

    #[test]
    pub fn math() {
        let source = r#"
            sqrt $r0 1
            trunc $r1 1.25
            ceil $r2 1.25
            floor $r3 1.25
            abs $r4 -5.2
            exp $r5 2.1
            inf $r6 1
            nan $r7 1
            max $r8 10 15
            min $r9 9 20
            log $r10 2 5
        "#;
        let mut vm = create_vm_from(source);

        while let VMStatus::Running = vm.tick().unwrap() {}

        assert_eq!(vm.registers()[0], 1.0f32.sqrt());
        assert_eq!(vm.registers()[1], 1.25f32.trunc());
        assert_eq!(vm.registers()[2], 1.25f32.ceil());
        assert_eq!(vm.registers()[3], 1.25f32.floor());
        assert_eq!(vm.registers()[4], (-5.2f32).abs());
        assert_eq!(vm.registers()[5], 2.1f32.exp());
        assert_eq!(vm.registers()[6], 0.0);
        assert_eq!(vm.registers()[7], 0.0);
        assert_eq!(vm.registers()[8], f32::max(10.0, 15.0));
        assert_eq!(vm.registers()[9], f32::min(9.0, 20.0));
        assert_eq!(vm.registers()[10], f32::log(5.0, 2.0));
    }
}

#[cfg(test)]
mod compilation_tests {
    use nano_risc_arch::{Limits, SourceUnit};
    use nano_risc_asm::{
        compiler::{self, CompilationErrorKind},
        parser,
    };

    #[test]
    fn duplicate_labels() {
        let source = r#"
            start:
                halt
            start:
                jmp start
        "#;

        let unit = SourceUnit::new_anonymous(source.as_bytes().to_vec());
        let tokens = parser::parse(&unit).unwrap();
        let assembly = compiler::compile(unit, tokens, None);

        assert_eq!(
            assembly.map_err(|err| err.kind().clone()),
            Err(CompilationErrorKind::DuplicateLabel {
                name: String::from("start")
            })
        );
    }

    #[test]
    fn max_size() {
        let source = r#"
            mov $r1 1
            mov $r1 1
            mov $r1 1
            mov $r1 1
            mov $r1 1
            mov $r1 1
            mov $r1 1
            mov $r1 1
            mov $r1 1
            mov $r1 1
        "#;

        let unit = SourceUnit::new_anonymous(source.as_bytes().to_vec());
        let tokens = parser::parse(&unit).unwrap();
        let assembly = compiler::compile(
            unit,
            tokens,
            Some(&Limits {
                max_assembly_length: 5,
                ..Default::default()
            }),
        );

        assert_eq!(
            assembly.map_err(|err| err.kind().clone()),
            Err(CompilationErrorKind::TooLargeAssembly { size: 5 })
        )
    }
}
