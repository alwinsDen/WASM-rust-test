import init,{run, test_logger} from "./pkg/WASM_Rust_Dev_Test.js"
init().then(()=>{
    run();
    test_logger();
})