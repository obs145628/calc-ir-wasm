extern crate binaryen;
extern crate wasmer_runtime;

mod ir;
mod parser;
mod runner;
mod wasmgen;

use std::env;

fn main() {
    let file_path = env::args().nth(1).expect("Usage: ./cl-jvm <input-file>");
    let ir_code = parser::parse_file(file_path.as_str());

    let wasm_mod = wasmgen::build_module(&ir_code);
    let wasm_bytes = wasm_mod.write();
    //wasm_mod.print();
    runner::exec_mod(&wasm_bytes);
}
