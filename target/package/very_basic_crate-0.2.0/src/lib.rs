//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// # Panics
/// The scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.
/// # Errors
/// If the function returns a `Result`, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
/// # Safety
///  If the function is `unsafe` to call, there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds two to the number given
///
/// # Examples
///
/// ```
/// let arg = 6;
/// let answer = my_crate::add_two(arg);
///
/// assert_eq!(8, answer);
/// ```

pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
    fn add_two_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
