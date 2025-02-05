//! Example docs for this crate

/// return example string
pub const fn example() -> &'static str {
    "alloy"
}

#[cfg(test)]
mod tests {
    use super::example;

    #[test]
    fn example_test() {
        assert_eq!(example(), "alloy");
    }
}
