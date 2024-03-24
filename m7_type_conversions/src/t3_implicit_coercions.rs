//!
//! # Implicit Coercions
//!
//! The explicit `as` casts are a superset of the implicit coercions that
//! the compiler will silently perform: any coercion can be forced with an
//! explicit `as`, but the converse is not true.
//!
//! In particular, the integral conversions performed are not coercions,
//! and so will always require `as`.
//!
//! Most of the coercions involve silent conversions of pointer and reference
//! types in ways that are sensible and convenient for the programmer, such as:
//!   • converting a mutable reference to a non-mutable references (so you can
//!     use a `&mut T` as the argument to a function that takes a `&T`)
//!   • converting a reference to a raw pointer (this isn't unsafe – the unsafety
//!     happens at the point where you're foolish enough to use a raw pointer)
//!   • converting a closure that happens not to capture any variables into a
//!     bare function pointer
//!   • converting an array to a slice
//!   • converting a concrete item to a "trait object", for a trait that the
//!     concrete item implements
//!   • converting an item lifetime to a "shorter" one.
//!
use std::ptr;

#[test]
fn removing_mutability_from_references_or_pointers() {
    let mut s = String::from("hello");
    let immute_ref: &String = &mut s;

    let p = &mut s as *mut String;
    let q: *const String = p;
}

#[test]
fn converting_references_to_raw_pionters() {
    let s = String::from("hello");
    let p: *const String = &s;

    let mut s = String::from("hello");
    let p: *mut String = &mut s;
}

#[test]
fn converting_non_capturing_closure_to_function_pointer() {
    let fp: fn(i32) -> i32 = |x| x + 1;
}

#[test]
fn converting_array_to_slice() {
    let slice: &[i32] = &[1, 2, 3, 4, 5];
}

#[test]
fn converting_concrete_items_to_trait_objects() {
    trait Fruit {
        fn name(&self) -> &str;
    }

    struct Apple;

    impl Fruit for Apple {
        fn name(&self) -> &str {
            "Apple"
        }
    }

    let fruit: &dyn Fruit = &Apple;
    println!("{}", fruit.name());
}

#[test]
fn shrinking_longer_lifetime_to_shorter_one() {
    fn choose_first<'a: 'b, 'b>(x: &'a str, _: &'b str) -> &'b str {
        x
    }

    let x = String::from("hello");
    {
        let y = String::from("world");
        let z = choose_first(&x, &y);
    }
}

#[test]
fn deref_coerions() {
    let boxed_string: Box<String> = Box::new(String::from("hello"));
    let inner: &str = &boxed_string;

    println!("{inner}");
}
