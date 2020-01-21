# calc-ir-wasm

This is a basic demo to demonstrate how to generate WASM binary in Rust.  
It takes as input a basic IR language with 5 operators on integers, and generate a wasm file that exports a function running the computation.

## Dependencies

This was tested with the following:
- Ubuntu 18.04
- node v12.9.1
- [emscripten](https://emscripten.org/) v1.39.6
- [binaryen](https://github.com/WebAssembly/binaryen) v90_36
- [wabt](https://github.com/WebAssembly/wabt) v1.0.13-2
- crate [bynaryen](https://github.com/pepyakin/binaryen-rs) v0.6.0
- crate [wasmer-runtime](https://github.com/wasmerio/wasmer-rust-example) v0.13.1

## cl-wasm

**TODO**  
The main rust project.  
Compiles IR files to .wasm files

# test-emcc-node

Basic test to compile C code and run with nodejs
