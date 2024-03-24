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

fn to_int(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

// TODO: Change the return type to `Option<String>`, and propagate a parsing error
//       to the caller instead of skipping failed elements.
fn sum_of_vecs(vs: Vec<String>) -> String {
    let mut acc = 0;

    for v in vs {
        acc += to_int(&v).unwrap_or(0);
    }

    acc.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_sum_vecs_when_all_elements_are_numeric_strings() {
        let vs = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        // assert_eq!(sum_of_vecs(vs), Some("6".to_string()));
    }

    #[test]
    fn should_return_none_if_any_of_element_is_non_numeric_string() {
        let vs = vec!["1".to_string(), "2abc".to_string(), "3".to_string()];

        // assert_eq!(sum_of_vecs(vs), None);
    }
}
