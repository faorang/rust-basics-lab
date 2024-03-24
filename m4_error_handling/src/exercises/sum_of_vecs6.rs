#![allow(unused)]

/**
 * Write a function `sum_of_vecs` that
 * 1. takes a vector of strings
 * 2. try to parse each string into an integer
 * 3. computes the sum of all these integers
 * 4. converts the sum into a string
 * 5. returns the string.
 *
 * Client should be able to handle the error case.
 */

#[derive(Debug)]
struct SummationError;

fn to_int(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

// TODO: Change the return type to `Result<String, SummationError>`, so that
//       this function propagate custom error type `SummationError` in case of
//       parsing failure.
fn sum_of_vecs(vs: Vec<String>) -> Option<String> {
    let mut acc = 0;

    for v in vs {
        acc += to_int(&v)?;
    }

    Some(acc.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::any::Any;

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

        // assert!(sum_of_vecs(vs).is_err());
    }

    #[test]
    fn should_return_error_if_any_of_element_is_non_numeric_string2() {
        let vs = vec!["1".to_string(), "2abc".to_string(), "3".to_string()];

        // let actual = sum_of_vecs(vs).unwrap_err();
        // let expected = SummationError;

        // assert_eq!(actual.type_id(), expected.type_id());
    }
}
