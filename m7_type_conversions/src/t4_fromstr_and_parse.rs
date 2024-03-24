//!
//! # ToString and Display
//!
//! The `ToString` trait allows for conversion to a `String` type.
//!
//! Rather than doing so directly, you should implement the `fmt::Display` trait
//! which automagically provides `ToString` (due to blanket implementation for
//! `T: Display`) and also allows printing the type.
//!
#![allow(non_snake_case)]

#[test]
fn conversion_to_String_by_implementing_ToString_trait() {
    struct Circle {
        radius: i32,
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

#[test]
fn converting_to_String_by_implementing_Display_trait() {
    use std::fmt;

    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

///
/// # FromStr and Parse
///
/// The `FromStr` trait allows for a type to define how it can be parsed from a string value.
/// The `parse()` method is available on any type that implements `FromStr`.
///
use std::str::FromStr;

#[test]
fn parsing_a_string() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[test]
fn test() {
    let expected = Ok(Point { x: 1, y: 2 });

    // Explicit call
    assert_eq!(Point::from_str("(1,2)"), expected);

    // Implicit calls, through `parse`
    assert_eq!("(1,2)".parse(), expected);
    assert_eq!("(1,2)".parse::<Point>(), expected);

    // Invalid input string
    assert!(Point::from_str("(1 2)").is_err());
}
