use std::fmt;

struct MyUnit;

impl fmt::Display for MyUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is the unit type: ()")
    }
}

fn main() {
    let unit = MyUnit;
    println!("{}", unit);  // 打印: This is the unit type: ()
}
