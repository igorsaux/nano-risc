use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use nano_risc_arch::{Limits, SourceUnit};
use nano_risc_asm::{compiler, parser};
use nano_risc_vm::{VMStatus, VM};

#[inline]
fn factorial(mut vm: VM) {
    loop {
        if matches!(vm.tick().unwrap(), VMStatus::Finished) {
            break;
        }
    }
}

fn create_vm() -> VM {
    let unit = SourceUnit::new(
        String::from("anonymous"),
        include_bytes!("../../factorial.asm").to_vec(),
    );
    let mut vm = VM::default();
    let tokens = parser::parse(&unit).unwrap();
    let assembly = compiler::compile(unit, tokens, &Limits::default()).unwrap();
    vm.load_assembly(assembly).unwrap();

    vm
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("factorial", move |b| {
        b.iter_batched(
            create_vm,
            |vm| factorial(black_box(vm)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
