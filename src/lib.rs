// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(a:i32) -> i32 {
  let mut b: i32 = 0;
  let mut c: i32 = 1;

  for _i in 0..a-1 {
    let temp: i32 = c;
    c = b + c;
    b = temp;
  }

  return c;
}