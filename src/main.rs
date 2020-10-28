fn fizzbuzz(input: u64) -> String {
    if 3 == input {
        "Fizz".to_string()
    } else {
        input.to_string()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fizzbuzz_1_test() {
        assert_eq!(fizzbuzz(1), 1.to_string());
    }

    #[test]
    fn fizzbuzz_3() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }
}
