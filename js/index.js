import("../pkg/index.js")
  .then((wasm) => {
    wasm.greet("WebAssembly");

    let buffer = new wasm.WasmMatrix(2, 2, array => {
      array[0] = 4;
      array[1] = 7;
      array[2] = 2;
      array[3] = 6;
    });
    console.log(buffer.get());

    let inv = buffer.inv();
    console.log(inv.get());

    buffer.free();
    inv.free();
  })
  .catch(console.error);
