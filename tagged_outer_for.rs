'outer: for x in 5..50 {
    for y in 0..10 {
        if x == y {
            println!("x == y == {x}");
            break 'outer;
        }
    }
}
