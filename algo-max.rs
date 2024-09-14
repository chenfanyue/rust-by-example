fn max(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &v in list {
        if v > largest {
            largest = v;
        }
    }

    largest
}
