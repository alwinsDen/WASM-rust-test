import init, {log_msg} from "./pkg/WASM_Rust_Dev_Test.js"
init().then(()=>{
    log_msg("This is an alert trigger from WASM");
})