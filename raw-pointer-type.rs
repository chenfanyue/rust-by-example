fn main() {
    let num: u8 = 42;
    let ptr: *const u8 = &num as *const u8;

    unsafe {
        println!("Value of num through ptr: {}", *ptr);
    }
}
