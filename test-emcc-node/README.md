Basic test that compile a simple c function to wasm, and call it from nodejs

# Dependencies

- emscripten
- binaryen (for optimizations)
- nodejs

# Build

```shell
emcc fact.c -O1 -s SIDE_MODULE=1 -s EXPORTED_FUNCTIONS="['_fact']" -o fact.wasm
wasm-opt -O3 fact.wasm -o fact.wasm
```

You can print the generated wasm code:
```
wasm-dis fact.wasm
```

# Run

```shell
node main.js
```
