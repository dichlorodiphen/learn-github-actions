pub fn increment(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_works() {
        assert_eq!(2, increment(1));
    }
}
