use lib::delim;

/**
 * A closure expression produces a closure value with a "unique", "anonymous" type
 * that cannot be written out. A closure type is approximately equivalent to a
 * struct which contains the captured variables.
 */

#[test]
fn type_inference() {
    // We cannot specify the type of the closure explicitly, because the compiler
    // can infer the type of the closure from the context.
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    // Uncomment this line to see what happens.
    // let n = example_closure(5); // Cannot pass i32 since the closure expects a String
}

/**
 * Capturing:
 *
 * Closures can capture variables:
 *
 *   | Borrowing immutably (by reference: &T)
 *   | Borrowing mutably (by mutable reference: &mut T)
 *   v Taking ownership (by value: T)
 *
 * They preferentially capture variables by reference and only go lower when required.
 */

#[test]
fn borrowing_immutably() {
    delim!();

    let list = vec![1, 2, 3];

    let only_borrows = || {
        // let m = list; // Uncomment this line to see what happens
        println!("From closure: {list:?}");
    };

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    delim!()
}

#[test]
fn borrowing_mutably1() {
    delim!();

    let mut list = vec![1, 2, 3];

    //  vvv <- note the `mut` keyword
    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {list:?}"); // Uncomment this line to see what happens
    borrows_mutably();
    println!("After calling closure: {list:?}");

    delim!();
}

#[test]
fn borrowing_mutably2() {
    delim!();

    let mut x = 42;

    let mut borrows_mutably = || x += 100;

    // println!("Before calling closure: {x:?}");
    borrows_mutably();
    println!("After calling closure: {x:?}");

    delim!();
}

#[test]
fn capturing_by_value1() {
    delim!();

    let list = vec![1, 2, 3];

    let take_ownership = || {
        println!("From closure: {list:?}");
        list // Change this to &list and see what happens ...
    };

    // println!("{list:?}"); // Error! borrow after move
    let vs = take_ownership();
    println!("vs: {vs:?}");

    delim!();
}

#[test]
fn capturing_by_value2() {
    delim!();

    let mut list = vec![1, 2, 3];

    //  vvv <- note the `mut` keyword
    let mut take_ownership = || {
        list.push(777);
        println!("From closure: {list:?}");
        list
    };

    // println!("{list:?}"); // Error! borrow after move
    let mut vs = take_ownership();
    println!("vs: {vs:?}");

    delim!();
}

#[test]
fn capturing_by_value3() {
    delim!();

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", *movable + 100);
        std::mem::drop(movable); // What if we comment this line out?
    };

    // println!("before consume, {movable:?}");

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.

    delim!();
}

#[test]
fn closure_capture_modes() {
    delim!();

    let mut list = vec![1, 2, 3];

    let only_borrows = || println!("From closure: {list:?}");

    // Can call `Fn` closure multiple times
    only_borrows();
    only_borrows();
    delim!();

    let mut borrows_mutably = || {
        list.push(7);
        println!("From closure: {list:?}");
    };

    // Can call `FnMut` closure multiple times
    borrows_mutably();
    borrows_mutably();
    delim!();

    let mut take_ownership = || {
        list.push(777);
        println!("From closure: {list:?}");
        list
    };

    // Can call `FnOnce` closure only once
    take_ownership();
    // take_ownership(); // Error: use of moved value: `take_ownership`

    delim!();
}

/**
 * Using `move` before vertical pipes forces closure to "take ownership of" captured variables.
 * If the `move` keyword is used, then all captures are "by move" or, for `Copy` types, "by copy",
 * regardless of whether a borrow would work.
 *
 * The `move` keyword is usually used to "allow the closure to outlive" the captured values,
 * such as if the closure is being returned or used to spawn a new thread.
 */
#[test]
fn capturing_via_explicit_move() {
    delim!();

    // `Vec` has non-copy semantics.
    let haystack: Vec<i32> = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There are {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow `haystack` variable immutably, hence `haystack` is still
    // available and uncommenting above line will not cause an error.
    delim!();
}

#[test]
fn capture_using_different_modes() {
    delim!();

    let x = String::from("Parker");
    let y = String::from("Rusty");
    let closure = {
        let x = &x;
        move || {
            println!("{:?}", "Hello, ".to_owned() + x); // borrowing (outer) x
            println!("{:?}", "hello, ".to_owned() + &y); // taking the ownership of y
        }
    };

    println!("{x:?}");
    // println!("{y}");

    closure();

    delim!();
}

#[test]
fn capture_using_different_modes2() {
    delim!();

    let s1 = String::from("A");
    let s2 = String::from("B");
    let s3 = String::from("C");

    let closure = {
        // `s1` is moved
        let s2 = s2.clone(); // `s2` is cloned
        let s3 = s3.as_str(); // `s3` is borrowed
        move || s1 + &s2 + s3
    };
    println!("{s2:?}, {s3:?}");
    println!("{:?}", closure()); // ABC

    delim!();
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

#[test]
fn partial_borrowing() {
    delim!();

    let mut alice_wonderland = Person {
        first_name: String::from("Alice"),
        last_name: String::from("Wonder"),
    };
    println!("{:?}", alice_wonderland);

    let print_first_name = || println!("First name: {}", alice_wonderland.first_name);
    alice_wonderland.last_name.push_str("land");
    print_first_name();

    println!("{:?}", alice_wonderland);

    delim!();
}
