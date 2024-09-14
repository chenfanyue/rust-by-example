#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        let mut i = 0;
        'inner: loop {
            println!("Entered the inner loop");

            if i == 3 {
                break 'outer;
            }
            i += 1;
            continue;
            println!("do sth else in the inner loop");
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
