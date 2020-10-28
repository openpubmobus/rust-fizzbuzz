fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {    
    //use super::*;    
    #[test]
    fn x_test() {
        assert_eq!(1, 2);
    }
}