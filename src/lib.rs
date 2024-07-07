use wasm_bindgen::prelude::*;

//defining entry point for WASM
#[wasm_bindgen(start)]
fn run() {
    bare_bones();
    using_a_macro();

    //test out the classes in JS module
    log(&format!("{}", name()));
    let mod_test = TestClass::new();
    assert_eq!(mod_test.number(), 42);
    mod_test.set_number(100);
    log(&format!("{}", mod_test.render()));
}

// draft and use external functions.
// non_browser_js.js contains sample functions.
#[wasm_bindgen(module = "/non_browser_js.js")]
extern "C" {
    fn name() -> String;
    type TestClass;

    #[wasm_bindgen(constructor)]
    fn new() -> TestClass;

    #[wasm_bindgen(method, getter)]
    fn number(this: &TestClass) -> u32;

    #[wasm_bindgen(method, setter)]
    fn set_number(this: &TestClass, n: u32) -> TestClass;

    #[wasm_bindgen(method)]
    fn render(this: &TestClass) -> String;
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// create a test log function
#[wasm_bindgen]
pub fn test_logger() {
    log("Synchronous load test");
}

// here we test 'em out
fn bare_bones() {
    log("test 1");
    log_u32(124142);
    log_many("alwin", "cool");
}

// logging using a macro
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "there");
    console_log!("this is a log test");
}

// working with char_type
#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32,
}

#[wasm_bindgen]
impl Counter {
    pub fn new(key: char, count: i32) -> Counter {
        log(&format!("Counter::new({}, {})", key, count));
        Counter { key, count }
    }

    pub fn key(&self) -> char {
        log("Counter.key()");
        self.key
    }

    pub fn count(&self) -> i32 {
        log("Counter.count");
        self.count
    }

    pub fn increment(&mut self) {
        log("Counter.increment");
        self.count += 1;
    }

    pub fn update_key(&mut self, key: char) {
        self.key = key;
    }
}
