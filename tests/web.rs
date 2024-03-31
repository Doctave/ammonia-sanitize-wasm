//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use ammonia_sanitize_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn sanitizes() {
    assert_eq!(
        &sanitize(
            "<div><script src='dangerous.js' /></div>".to_string(),
            SanitizeOptions {}
        ),
        "<div></div>"
    );
}
