// 所有权、可变借用、不可变借用
fn main() {
    let a = "ab".to_string();
    println!("{a:?}");
    println!("{:?}", &a as *const String);      // 打印栈上 String 结构体的地址
    println!("{:?}", a.as_ptr());               // 打印堆上实际字符串数据的地址

    (|| {
        println!("{a:?}");
        println!("{:?}", &a as *const String);  // 新的栈上 String 结构体的地址
        println!("{:?}", a.as_ptr());           // 打印堆上实际字符串数据的地址
    })();

    println!("{a:?}");
}

fn main() {
    let mut a = "ab".to_string();
    println!("{a:?}");
    println!("{:?}", &a as *const String);      // 打印栈上 String 结构体的地址
    println!("{:?}", a.as_ptr());               // 打印堆上实际字符串数据的地址

    (|| {
        a.push('c');
        println!("{a:?}");
        println!("{:?}", &a as *const String);  // 新的栈上 String 结构体的地址
        println!("{:?}", a.as_ptr());           // 打印堆上实际字符串数据的地址
    })();

    println!("{a:?}");
}

// move 表示对外部变量按值捕获，捕获的过程涉及到所有权的转移。然后就看外部变量有没有实现 Copy trait ，如果没有实现就把原值的所有权转移，如果实现了其实转移的是副本的所有权
fn main() {
    let mut a = "ab".to_string();
    println!("{a:?}");
    println!("{:?}", &a as *const String);      // 打印栈上 String 结构体的地址
    println!("{:?}", a.as_ptr());               // 打印堆上实际字符串数据的地址

    (move || {
        a.push('c');
        println!("{a:?}");
        println!("{:?}", &a as *const String);  // 新的栈上 String 结构体的地址
        println!("{:?}", a.as_ptr());           // 打印堆上实际字符串数据的地址
    })();

    // println!("{a:?}");
}

fn main() {
    let mut a = 1i32;
    println!("{a:?}");
    println!("{:?}", &a as *const i32);

    (move || {
        a += 1000;
        println!("{a:?}");
        println!("{:?}", &a as *const i32);
    })();

    println!("{a:?}");
}




fn main() {
    let s = String::from("hello");
    let consume_string = move || println!("{}", s);

    consume_string();
    consume_string();
    // println!("{s}");
}


fn consume_with_relish<F>(func: F)
where F: FnOnce() -> String
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());
}
fn main() {
    let x = String::from("x");
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);
    // `consume_and_return_x` can no longer be invoked at this point
    consume_with_relish(consume_and_return_x);
}


