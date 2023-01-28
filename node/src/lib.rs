//!
//! Framework compoents for using Node.js and NWJS in WASM environment
//! 
use wasm_bindgen::prelude::*;

pub mod error;
pub mod process;
pub mod result;

pub mod prelude {
    pub use crate::process::*;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub fn require(s: &str) -> JsValue;
}
