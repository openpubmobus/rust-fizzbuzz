fn fizzbuzz (input: u64) -> String {
    "0".to_string()
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
}