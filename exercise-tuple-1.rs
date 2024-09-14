use std::fmt;

fn transpose(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix)
}
