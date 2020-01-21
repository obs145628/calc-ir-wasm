use wasmer_runtime::{imports, instantiate, Value};

pub fn exec_mod(wasm_bytes: &[u8]) {
    let import_object = imports! {};
    let env = instantiate(wasm_bytes, &import_object).expect("Failed to instantiate wasm module");
    let res = env.call("run", &[]).expect("Failed to call function");
    let res = res.get(0).expect("Missing return value");
    match res {
        Value::I32(x) => println!("{}", x),
        _ => print!("Expected int32 return type"),
    }
}
