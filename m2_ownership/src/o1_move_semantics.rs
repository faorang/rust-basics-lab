/*
 * Move semantics:
 * The important thing to note here is that, by default, assignments move ownership.
 * This means that when we assign a value to a variable, we're transferring ownership.
 */

// #[derive(Debug)] instructs the compiler to auto-generate the code needed to
// satisfy the Debug trait (think interface). We need this to be able to print
// out the values
#[derive(Debug)] // just so we can print out User
struct User {
    id: u32,
}

#[test]
pub fn move_ownership_on_assignment() {
    let u1 = User { id: 9000 };
    println!("{u1:?}");

    let u2 = u1; // `u1` is moved to `u2`
    println!("{u2:?}");

    // this is an error
    // println!("{:?}", u1);
}

#[test]
pub fn move_ownership_on_parameter_passing() {
    fn print_user(u: User) {
        println!("{u:?}");
    }

    let u = User { id: 9000 };
    print_user(u); // `u` is moved into the function

    // this is an error
    // println!("{u:?}");
}

#[test]
pub fn give_and_take_under_move() {
    fn print_user(u: User) -> User {
        println!("{u:?}");
        u
    }

    let u = User { id: 9000 };
    let u = print_user(u);
    println!("{u:?}");
}
