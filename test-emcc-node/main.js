const fs = require('fs').promises;
const util = require('util');

async function run() {
  async function createWebAssembly(bytes) {
    const memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });
    const env = {
      abortStackOverflow: (err) => { throw new Error(`overflow: ${err}`); },
      table: new WebAssembly.Table({ initial: 0, maximum: 0, element: 'anyfunc' }),
      __table_base: 0,
      memory,
      __memory_base: 1024,
      STACKTOP: 0,
      STACK_MAX: memory.buffer.byteLength,
    };
    return WebAssembly.instantiate(bytes, { env });
  }

  const lib = await createWebAssembly(new Uint8Array(await fs.readFile('./fact.wasm')));
  console.log(util.inspect(lib, true, 0));


  const fact = lib.instance.exports.fact;
  console.log(fact.toString());
  for (var i = 0; i < 10; ++i)
    console.log(i + " => " + fact(i));
}
run();
