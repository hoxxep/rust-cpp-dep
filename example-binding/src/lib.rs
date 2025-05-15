#[link(name = "stuff_from_cpp", kind = "static")]
extern "C" {
    fn example_cpp(input: u64) -> u64;
}

pub fn example_rust(input: u64) -> u64 {
    unsafe { example_cpp(input) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = 42;
        let result = example_rust(input);
        assert_eq!(result, input * 64);
    }
}