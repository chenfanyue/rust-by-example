for i in std::iter::repeat(5) {
    println!("turns out i={i} never stops being 5");
    break; // would loop forever otherwise
}
