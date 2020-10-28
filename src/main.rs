fn fizzbuzz(input: u64) -> String {
    match (input % 3, input % 5) {
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => input.to_string()
    }

//     if input % 3 == 0 {
//         "Fizz".to_string()
//     } else if input % 5 == 0 {
//         "Buzz".to_string()
//     } else {
//         input.to_string()
//     }
}

fn main() {
    println!("Hello, world!");
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
}
