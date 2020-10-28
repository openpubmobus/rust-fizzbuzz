fn fizzbuzz(input: u64) -> String {
    match (input % 3, input % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fizzbuzz_1() {
        assert_eq!(fizzbuzz(1), 1.to_string());
    }

    #[test]
    fn fizzbuzz_multiple_of_3() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(6), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn fizzbuzz_multiple_of_5() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(10), "Buzz");
    }

    #[test]
    fn fizzbuzz_multiple_of_3_and_5() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn fizzbuzz_1_to_100() {
        assert_eq!(fizzbuzzrange(3), "1 2 Fizz");
    }
}
