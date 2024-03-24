#![allow(unused)]
///
/// Rust type conversions fall into three categories:
///
/// • "semi-automatic": explicit casts between values using the `as` keyword
/// • "automatic": implicit coercion into a new type.
/// • "manual": user-defined type conversions provided by implementing the `From` and `Into` traits
///
fn main() {
    let x: i16 = 42;
    // let y: i32 = x;
}
