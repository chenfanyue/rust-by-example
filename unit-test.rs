pub fn greeting(name: &str) -> String {
    format!("hi {}", name)
}

#[cfg(test)]
mod tests {
    use crate::greeting; // use super::*;

    #[test]
    fn greeting_contains_name() {
        let s = greeting("carol");
        assert!(s.contains("carol"));
    }
}
