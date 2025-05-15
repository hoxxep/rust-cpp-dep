fn mul64(input: u64) -> u64 {
    example_binding::example_rust(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mul64(10), 640);
    }
}
