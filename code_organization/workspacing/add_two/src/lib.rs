pub fn add_two(v: usize) -> usize {
    v + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(4);
        assert_eq!(result, 6);
    }
}
