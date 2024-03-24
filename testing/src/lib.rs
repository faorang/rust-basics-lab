//!
//! # Examples
//! ```
//! let result = testing::add(2, 2);
//! assert_eq!(result, 4);
//! ```

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

///
/// # Examples
/// ```
/// let result = testing::add_two(2);
/// assert_eq!(result, 4);
/// ```

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_with_two_using_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_using_add_two() {
        assert_eq!(10, add_two(8));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore] // "cargo test -- --ignored" to run ignored tests
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    #[should_panic(expected = "will panic")]
    fn should_panic() {
        panic!("This test will panic");
    }

    #[test] // cargo test -- --show-output
    fn should_print_to_screen() {
        println!("This will print to the screen ...............");
    }
}
