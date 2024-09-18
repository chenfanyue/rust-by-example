fn f(mut x: Vec<i32>) -> Vec<i32> {
    x.push(2);
    x
}

fn main() {
    let x = vec![1,];
    println!("{:?}", f(x));
    println!("{:?}", x);
}


fn f(mut x: Vec<i32>) -> Vec<i32> {
    x.push(2);
    x
}

fn main() {
    let x = vec![1,];
    println!("{:?}", f(x.clone()));
    println!("{:?}", x);
}


fn f(x: &mut Vec<i32>) -> () {
    x.push(2);
}

fn main() {
    let mut x = vec![1];
    println!("{:?}", f(&mut x));
    println!("{:?}", x);
}
