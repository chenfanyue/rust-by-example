fn main() {
    let x: i32 = {
        let y = 2;
        y + 1
    };
    println!("{}", x);
}
