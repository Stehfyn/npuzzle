/**
 * @file npuzzle.rs
 *
 * @brief This is the module for creating an interface to javascript functions found in index.html.
 *
 * @author Stephen Foster
 * Contact: stephenfoster@nevada.unr.edu
 *
 */
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn isIOS() -> bool;
    #[wasm_bindgen(js_namespace = window)]
    pub fn isMobile() -> bool;
    #[wasm_bindgen(js_namespace = window)]
    fn getMouseX() -> f64;
    #[wasm_bindgen(js_namespace = window)]
    fn getMouseY() -> f64;
}
