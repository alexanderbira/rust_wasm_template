import init from "./pkg/rust_wasm_template.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const pkg = await init("./pkg/rust_wasm_template_bg.wasm");

  const n = prompt("Enter the index of the Fibonacci you want: ");

  const addResult = pkg.fib(n);

  document.body.textContent = `Fibonacci number #${n}: ${addResult}`;
}

runWasm();