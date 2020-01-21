The program READS an IR file, then uses binaryen API to convert it to a binary wasm code.  
The WASM module is loaded with WASMER, and the computation function is called.

# Build Compiler

```shell
cargo build
```

# Run compiler

```shell
cargo run ./tests/compute.ir
```

The output should be `-2`
