fn main() {
    let num: u8 = 42;
    let ptr: *const u8 = &num as *const u8;

    unsafe {
        println!("Value of num through ptr: {}", *ptr);
    }
}

fn main() {
    let a = "ab".to_string();
    println!("{a:?}");
    println!("{:?}", &a as *const String);  // 打印栈上 String 结构体的地址
    println!("{:?}", a.as_ptr());           // 打印堆上实际字符串数据的地址

    (|a: String| {
        println!("{a:?}");
        println!("{:?}", &a as *const String);  // 新的栈上 String 结构体的地址
        println!("{:?}", a.as_ptr());           // 打印堆上实际字符串数据的地址
    })(a);
}
