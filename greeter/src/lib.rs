#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contains name: {}, value was {}",
            name, result,
        );
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello")
}
