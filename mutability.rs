fn f(mut x: i32) -> i32 {
    x += 1;
    x
}

fn main() {
    let x = 0;
    println!("{}", f(x));
}
