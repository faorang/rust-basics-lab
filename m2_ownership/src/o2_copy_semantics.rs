/*
 * Copy semantics:
 * The important thing to note is that while assignments are, by default, a move,
 * we can change that behaviour, on a type by type basis, to be a copy.
 */

#[derive(Debug)] // just so we can print out User
struct User {
    id: u32,
}

#[test]
pub fn copy_semantics() {
    let u1 = 9001;
    println!("{u1:?}");

    let u2 = u1; // `u1` is copied to `u2`
    println!("{u2:?}");

    // with an integer, this works
    println!("{u1:?}");
}

impl Clone for User {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for User {}

#[test]
pub fn now_copy_is_possible() {
    let u1 = User { id: 9000 };
    println!("{u1:?}");

    let u2 = u1; // `u1` is copied to `u2`
    println!("{u2:?}");

    // this now works
    println!("{u1:?}");
}
