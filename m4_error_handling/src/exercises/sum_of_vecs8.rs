#![allow(unused)]

use std::num::ParseIntError;

/**
 * Write a function `sum_of_vecs` that
 * 1. takes a vector of strings
 * 2. try to parse each string into an integer
 * 3. computes the sum of all these integers
 * 4. converts the sum into a string
 * 5. returns the string.
 *
 * Propagate parse results all the way up.
 */

#[derive(Debug)]
struct SummationError;

fn to_int(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

// TODO: Change the return type to `Result<i32, SummationError>` so that
//       it propagates parse results with custom error all the way up.
fn sum_of_vecs(vs: Vec<String>) -> Result<String, ParseIntError> {
    let mut acc = 0;

    for v in vs {
        acc += to_int(&v)?;
    }

    Ok(acc.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{any::Any, num::IntErrorKind};

    #[test]
    fn should_sum_vecs_when_all_elements_are_numeric_strings() {
        let vs = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        let actual = sum_of_vecs(vs).unwrap();
        let expected = "6".to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_return_error_if_any_of_element_is_non_numeric_string() {
        let vs = vec!["1".to_string(), "2abc".to_string(), "3".to_string()];

        assert!(sum_of_vecs(vs).is_err());
    }

    #[test]
    fn should_return_error_if_any_of_element_is_non_numeric_string2() {
        let vs = vec!["1".to_string(), "2abc".to_string(), "3".to_string()];

        let actual = sum_of_vecs(vs).unwrap_err();
        let expected = SummationError;

        assert_eq!(actual.type_id(), expected.type_id());
    }
}
