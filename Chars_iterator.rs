fn main() {
    let s = String::from("Hello");
    let mut it = s.chars();
    it.next();
    for c in it {
        println!("{}", c);
    }
    println!("{}",s);
}
