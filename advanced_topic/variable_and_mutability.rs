fn main() {
    let mut x = 1;
    let mut y = 100;

    let mut r = &mut x;
    *r = 10;
    r = &mut y;
    *r = 1000;

    println!("{}", x);
    println!("{}", y);
}
