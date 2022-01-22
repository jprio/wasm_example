use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[no_mangle]
#[wasm_bindgen]
pub fn will_return_string() -> String {
    String::from("Hello from Rust")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
