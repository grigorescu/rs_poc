#[no_mangle]
pub extern "C" fn rust_function(len: u32) {
    println!("Rust saw a packet of size {}", len);
}
