
extern crate nalgebra as na;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Float32Array;
use js_sys::Function;
use na::{DMatrix};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("Hello world!"));
    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&JsValue::from_str(&format!("Hello, {}!", name)));
}

#[wasm_bindgen]
pub struct WasmMatrix {
    rows: usize,
    cols: usize,
    buffer: Vec<f32>,
}

#[wasm_bindgen]
impl WasmMatrix {
    #[wasm_bindgen(constructor)]
    pub fn new(rows: i32, cols: i32, f: &Function) -> Self {
        let mut buffer = vec![0f32; (rows * cols) as usize];
        unsafe {
            let array = Float32Array::view(&mut buffer);
            f.call1(&JsValue::NULL, &JsValue::from(array))
                .expect("The callback function should not throw");
        }
        Self { buffer, rows: rows as usize, cols: cols as usize }
    }

    #[wasm_bindgen]
    pub fn inv(&self) -> Result<WasmMatrix, JsValue> {
        let m = DMatrix::from_row_slice(self.rows, self.cols, &self.buffer);
        match m.try_inverse() {
            Some(inv) => {
                unsafe {
                    Ok(Self {
                        // TODO review so many reallocations
                        buffer: inv.transpose().as_slice().to_vec(),
                        rows: inv.nrows(),
                        cols: inv.ncols()
                    })
                }
            }
            None => {
                Err(JsValue::from_str("Matrix is not inversible"))
            }
        }
    }

    #[wasm_bindgen]
    pub fn get(&self) -> Float32Array {
        unsafe {
            Float32Array::view(self.buffer.as_slice())
        }
    }
}