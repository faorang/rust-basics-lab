#![allow(unused)]

/**
 * Write a function `sum_of_vecs` that
 * 1. takes a vector of strings
 * 2. try to parse each string into an integer
 * 3. computes the sum of all these integers
 * 4. converts the sum into a string
 * 5. returns the string.
 *
 * Hint:
 * - Use `parse::<i32>()` to convert a `&str` to an `i32`.
 * - `String` implements `Deref`, so you can treat a `String` like a `&str`.
 */

// TODO: Change `unwrap()` to `expect()` to make the test pass.
fn to_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn sum_of_vecs(vs: Vec<String>) -> String {
    let mut acc = 0;

    for v in vs {
        acc += to_int(&v);
    }

    acc.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_sum_vecs_when_all_elements_are_numeric_strings() {
        let vs = vec!["1".to_string(), "2".to_string(), "3".to_string()];

        assert_eq!(sum_of_vecs(vs), "6");
    }

    #[test]
    #[should_panic(expected = "Element is not a numeric string")]
    fn should_panic_if_vec_contains_any_non_numeric_string() {
        let vs = vec!["1".to_string(), "2abc".to_string(), "3".to_string()];

        assert_eq!(sum_of_vecs(vs), "6");
    }
}