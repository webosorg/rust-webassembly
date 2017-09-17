fn main() {
    println!("Welcome to playground for Rust and WebAssembly");
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b
}