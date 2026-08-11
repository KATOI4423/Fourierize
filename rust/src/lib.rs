//! lib.rs
//!
//! Define front-end API

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn create_fourier_series_lettering(pixels: &[u8], size: u16, orders: &[u8]) {
    console::log_1(&format!("pixels.len() = {}", pixels.len()).into());
    console::log_1(&format!("size = {}", size).into());
    console::log_1(&format!("orders.len() = {}", orders.len()).into());
}
