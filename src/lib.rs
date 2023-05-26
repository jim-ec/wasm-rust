use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> JsValue {
    log("Hello world from Rust");
    console_log!("foo({a}, {b})");
    let c = a + b;
    JsValue::from(c)
}

#[wasm_bindgen]
pub fn length(a: Vec<JsValue>) {
    log("Hello world");
    console_log!("length({a:?})");
}
