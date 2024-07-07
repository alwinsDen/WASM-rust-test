//test export function
export function name(){
    return "Message from independent Rust module";
}

export class TestClass {
    constructor(){
        this._number = 42;
    }
    get number(){
        return this._number;
    }

    //WASM compiler failed if set_ is not added before functions name.
    set number(n){
        return this._number = n;
    }
    render(){
        return `Test number is: ${this.number}`;
    }
}