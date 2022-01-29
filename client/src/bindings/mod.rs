use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/bindings/copy-to-clipboard.js")]
extern "C" {
  #[wasm_bindgen(js_name = "copyToClipboard")]
  pub fn copy_to_clipboard(text: String);
}
