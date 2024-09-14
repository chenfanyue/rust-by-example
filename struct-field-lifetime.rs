struct Ex<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let s = Ex {
        part: &string1,
    };

    println!("part is {}", s.part);
}
