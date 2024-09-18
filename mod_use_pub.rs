mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn _subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    use crate::math::add;

    let result = add(5, 3);
    println!("Result: {}", result);
}



mod math {
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }
}

fn main() {
    let p = math::Point::new(5, 10);
    println!("Point: ({}, {})", p.x, p.y);
}



// exercise.rs
pub mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner module!");
        }
    }
}

// main.rs
mod exercise;

use exercise::outer::inner::greet;

fn main() {
    greet();
}



// directory tree
my_project/
└── src
    ├── main.rs      # crate root
    └── utils/
        └── math.rs

// math.rs
pub(crate) fn add(a: i32, b: i32) -> i32 {
    a + b
}

// main.rs
mod utils {
    pub(crate) mod math;
}

fn main() {
    let x = utils::math::add(1, 2);
    println!("{x}");
}
