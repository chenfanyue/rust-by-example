// 所有权、可变借用、不可变借用

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


