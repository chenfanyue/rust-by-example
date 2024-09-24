fn main() {
    let elem = 5u8;
    let mut vec: Vec<u8> = Vec::new();
    vec.push(elem);

    println!("{:?}", vec);
}

fn main() {
    let v = Vec::from_iter(0..2);

    assert_eq!(vec![0, 1], v);
    assert_eq!(2, v.len());
}
