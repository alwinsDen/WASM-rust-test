use wasm_bindgen::prelude::*;

// DEF: here is the trait definition
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn log_msg(s: &str){
    alert(&format!("{}!", s));
}