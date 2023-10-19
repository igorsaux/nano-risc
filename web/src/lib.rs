use nano_risc_arch::{Assembly, Limits, SourceUnit};
use nano_risc_asm::{compiler, parser};
use nano_risc_vm::{VMStatus, Value, VM};
use serde::{Deserialize, Serialize};
use std::{panic, rc::Rc};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramError {}

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
pub fn vm_load_assembly(handle: usize, code: String) -> JsValue {
    let vm = unsafe { &mut *(handle as *mut VM) };
    let unit = SourceUnit::new_anonymous(code.as_bytes().to_vec());

    let tokens = match parser::parse(&unit) {
        Ok(tokens) => tokens,
        Err(error) => return serde_wasm_bindgen::to_value(&error).unwrap(),
    };

    let assembly = match compiler::compile(unit, tokens) {
        Ok(assembly) => assembly,
        Err(error) => return serde_wasm_bindgen::to_value(&error).unwrap(),
    };

    match vm.load_assembly(assembly) {
        Ok(_) => {}
        Err(error) => return serde_wasm_bindgen::to_value(&error).unwrap(),
    }

    JsValue::NULL
}

#[wasm_bindgen]
pub fn vm_pc_to_location(handle: usize) -> JsValue {
    let vm = unsafe { &mut *(handle as *mut VM) };

    if let Some(Assembly {
        debug_info: Some(debug_info),
        ..
    }) = vm.assembly()
    {
        let Some(location) = debug_info.source_loc.get(&vm.pc()) else {
            return JsValue::NULL;
        };

        serde_wasm_bindgen::to_value(&location).unwrap()
    } else {
        JsValue::NULL
    }
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
            Value::Float { value } => array.push(&JsValue::from_f64(*value as f64)),
            Value::String { value } => array.push(&JsValue::from_str(value)),
        };
    }

    array
}
