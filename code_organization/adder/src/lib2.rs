//! # Adder
//!
//! `adder` is a collection of utilities to make performing certain calculations easier.

/// Adds two numbers together and return the result
/// # Examples
///
/// ```
/// let arg1 = 3;
/// let arg2 = 4;
///
/// let result = adder::add(arg1, arg2);
///
/// assert_eq!(7, result);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
