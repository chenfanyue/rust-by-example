fn main() {
    let v = vec![1, 2, 3];
    let s = &v[..];
    for i in s {
        println!("{}", i); // 输出每个元素: 1 2 3
    }
    println!("{:?}", v);
}
