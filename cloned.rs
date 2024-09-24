let a = [vec![0_u8, 1, 2], vec![3, 4], vec![23]];

let faster: Vec<_> = a.iter().filter(|s| s.len() == 1).cloned().collect();
assert_eq!(&[vec![23]], &faster[..]);
