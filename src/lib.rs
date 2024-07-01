use wasm_bindgen::prelude::*;

//defining entry point for WASM
#[wasm_bindgen(start)]
fn run() {
    bare_bones();
    using_a_macro();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn log_many(a: &str, b: &str);
}

//here we test 'em out
fn bare_bones() {
    log("test 1");
    log_u32(321);
    log_many("alwin", "cool")
}

//loggging using a macro
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "there");
    console_log!("this is a log test");
}