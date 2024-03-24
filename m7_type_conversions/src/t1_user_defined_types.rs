//!
//! # Conversoins between user defined types
//!
//! The four relevant traits that express the ability to convert values of a type are:
//!   • From<T>: Items of this type can be built from items of type T.
//!   • TryFrom<T>: Items of this type can sometimes be built from items of type T.
//!   • Into<T>: Items of this type can converted into items of type T.
//!   • TryInto<T>: Items of this type can sometimes be converted into items of type T.
//!
//! The type conversion traits have an obvious symmetry:
//! if T ⟾ U is possible, it should also be possible to U ⟾ T.
//!
//! Advice: "implement the `From` trait for conversions.
//!
//! If you need to define a new generic of your own as one of these two traits as a
//! trait bound, then the advice is reversed: "use the `Into` trait for trait bounds".
//! That way, the bound will be satisfied both by things that directly implement `Into`,
//! and by things that only directly implement `From`.
//!
//! Blanket trait implementation:
//! "I can implement Into<U> for a type T whenever U already implements From<T>".
//!
//! ```
//!   impl<T, U> Into<U> for T where U: From<T> {
//!       fn into(self) -> U {
//!           U::from(self)
//!       }
//!   }
//! ```
//!
//! Reflexive implementation: "I can implement From<T> for a type T".
//! ```
//! `
//!   impl<T> From<T> for T {
//!       fn from(t: T) -> T {
//!           t
//!       }
//!   }
//! ```
//!
//! For consistency and safety you should prefer from / into conversions to `as` casts.
//!

/*
* From and Into
*/
use std::convert::{From, Into};

#[test]
fn basic_from_test() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);
}

#[test]
fn basic_into_test() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl Into<Number> for i32 {
        fn into(self) -> Number {
            Number { value: self }
        }
    }

    let int = 5;

    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[test]
fn once_from_is_implemented_into_can_be_automatically_generated_via_blanket_implementation() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let int = 5;

    // Once `From` is implemented for `Number`,
    // `Into` is automatically implemented for `Number`
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[test]
fn recommended_usage_patterns() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    // Implement `From` trait for `Number`
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    // Bad API: don't do this
    fn bad_api(n: Number) {
        println!("My number is {:?}", n);
    }

    // Define a generic function `foo` using `Into`
    fn good_api<T: Into<Number>>(n: T) {
        let n = n.into();
        println!("My number is {:?}", n);
    }

    bad_api(42.into()); // Do not make clients do this
    good_api(42);

    let number = Number::from(42);
    good_api(number); // Ok since reflection implementation for `From`
}

/*
 * TryFrom and TryInto
 */
use std::convert::{TryFrom, TryInto};

#[test]
fn try_from_demo() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 1_942_i32.try_into();
    assert_eq!(result, Ok(EvenNumber(1_942)));
    let result: Result<EvenNumber, ()> = 1_569_i32.try_into();
    assert_eq!(result, Err(()));
}
