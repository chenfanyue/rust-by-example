struct Ex<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("long long ago. a cat play");
    let first_sentence = novel.split('.').next().expect("sentence not exist");
    let s = Ex {
        part: first_sentence,
    };

    println!("{}", s.part);
}
