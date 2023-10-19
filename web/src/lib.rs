use parser::Parser;
use std::{panic, rc::Rc};
use vm::{Limits, Program, VMStatus, VM};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn vm_create(limits: JsValue) -> usize {
    let limits: Limits = serde_wasm_bindgen::from_value(limits).unwrap_or_default();

    Rc::into_raw(Rc::new(VM::new(limits))) as usize
}

#[wasm_bindgen]
pub fn vm_release(handle: usize) {
    unsafe { Rc::decrement_strong_count(handle as *const VM) }
}

#[wasm_bindgen]
pub fn vm_set_dbg_callback(handle: usize, callback: js_sys::Function) {
    let vm = unsafe { &mut *(handle as *mut VM) };

    vm.set_dbg_callback(Box::new(move |text| {
        let value = JsValue::from_str(&text);
        callback.call1(&JsValue::UNDEFINED, &value).unwrap();
    }))
}

#[wasm_bindgen]
pub fn vm_load_program(handle: usize, code: String) {
    let vm = unsafe { &mut *(handle as *mut VM) };
    let assemby = Parser::new_string(code).parse().unwrap();
    let program = Program::try_compile(assemby).unwrap();

    vm.load_program(program);
}

#[wasm_bindgen]
pub fn vm_get_pc(handle: usize) -> usize {
    let vm = unsafe { &mut *(handle as *mut VM) };

    vm.pc()
}

#[wasm_bindgen]
pub fn vm_tick(handle: usize) -> usize {
    let vm = unsafe { &mut *(handle as *mut VM) };

    match vm.tick().unwrap() {
        VMStatus::Idle => 0,
        VMStatus::Yield => 1,
        VMStatus::Running => 2,
        VMStatus::Finished => 3,
    }
}

#[wasm_bindgen]
pub fn vm_reset(handle: usize) {
    let vm = unsafe { &mut *(handle as *mut VM) };

    vm.reset();
}

#[wasm_bindgen]
pub fn vm_get_registers(handle: usize) -> js_sys::Array {
    let vm = unsafe { &mut *(handle as *mut VM) };
    let array = js_sys::Array::new();

    for register in vm.registers() {
        match register {
            vm::Value::Float { value } => array.push(&JsValue::from_f64(*value as f64)),
            vm::Value::String { value } => array.push(&JsValue::from_str(value)),
        };
    }

    array
}
