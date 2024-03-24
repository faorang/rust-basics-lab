use lib::delim;

/**
 * A closure expression produces a closure value with a unique,
 * anonymous type that cannot be written out.
 */

#[test]
fn what_is_my_type() {
    let count = 0;

    let print_count_closure = || println!("Count value: {}", count);

    delim!();
    print_count_closure();
    delim!();
}

/**
 * Similarly, we cannot specify the type of closure argument in a function definition.
 * Thus, the only way to define that a function should accept a closure as an argument
 * is through "trait bounds".
 *
 * Rust provides three different traits `Fn`, `FnMut`, and `FnOnce` that can be used as
 * trait bounds for closure arguments. Each closure implements one or many of these three
 * traits, and what trait is automatically implemented depends on how the closure captured
 * environment is used in the closure body.
 */

#[test]
fn fn_closure() {
    delim!();

    // A fuction that accepts `Fn` closure as an argument.
    fn accept<F: Fn(&str)>(f: F) {
        f("fst");
        f("snd");
    }

    let immutable = String::from("Rusty");

    let closure = |arg: &str| {
        println!("{arg}: {}", immutable.len());
    };

    accept(closure); // copied
    delim!();

    accept(closure); // copied
    delim!();

    println!("{immutable:?}");
    closure("closure can be called again");
    delim!();
}

#[test]
fn fn_mut_closure() {
    delim!();

    // A function that accepts `FnMut` closure as an argument.
    fn accept<F: FnMut(&str)>(mut f: F) {
        // Note: `mut f`
        f("fst");
        f("snd");
    }

    let mut mutable = String::from("Rusty");

    let mut closure = |arg: &str| {
        mutable.push_str(" is mutable");
        println!("{arg}: Inside closure: {mutable}");
    };

    accept(closure); // moved
    delim!();
    // accept(closure); // use of moved value: `closure`

    println!("{mutable:?}");
    delim!();
}

#[test]
fn fn_once_closure() {
    delim!();

    // A function that accepts `FnOnce` closure as an argument.
    fn accept<F: FnOnce(&str) -> String>(f: F) {
        // `f` and `mut f` are both ok
        f("only once");
        // f("only once"); // use of moved value: `f`
    }

    let mut mutable = String::from("Rusty");

    let closure = |arg: &str| {
        mutable.push_str(" is moved");
        println!("{arg}: Inside closure: {mutable}");
        mutable
    };

    accept(closure);
    // accept(closure); // use of moved value: `closure`
    // println!("{mutable:?}"); // use of moved value: `mutable`
    delim!();
}

/**
 * Closures may implement multiple traits: Fn, FnMut, FnOnce.
 */

fn accept_fn_once<F: FnOnce() -> String>(f: F) -> String {
    println!("Accepted by FnOnce");
    f()
}

fn accept_fn_mut<F: FnMut() -> String>(mut f: F) -> String {
    println!("Accepted by FnMut");
    f()
}

fn accept_fn<F: Fn() -> String>(f: F) -> String {
    println!("Accepted by Fn");
    f()
}

#[test]
fn closures_may_implement_multiple_traits_case_for_fn_closure() {
    let x = 42;

    let closure = || {
        println!("======> I'm an Fn closure! {x}");
        "I'm a closure!".to_owned()
    };

    delim!();
    accept_fn(closure); // ok
    accept_fn_mut(closure); // ok
    accept_fn_once(closure); // ok
    delim!();
}

#[test]
fn closures_may_implement_multiple_traits_case_for_fn_mut_closure() {
    let mut x = 42;

    let mut closure = || {
        x += 1;
        println!("======> I'm a FnMut closure! {x}");
        "I'm a closure!".to_owned()
    };

    delim!();
    // accept_fn(closure); // oops!
    // accept_fn_mut(closure); // ok
    accept_fn_once(closure); // ok
    delim!();
}

#[test]
fn closures_may_implement_multiple_traits_case_for_fn_once_closure() {
    let mut x = String::from("hello");

    let mut closure = || {
        x.push_str(", World");
        println!("I'm a closure! {x}");
        x
    };

    delim!();
    // accept_fn(closure); // oops!
    // accept_fn_mut(closure); // oops!
    accept_fn_once(closure);
    delim!();
}

#[test]
fn quiz_what_should_be_the_type_of_the_closure_diary() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `_______________`.
        println!("I said {greeting}.");

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `______________`.
        farewell.push_str("!!!");
        println!("Then I screamed {farewell}.");
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `_____________`.
        mem::drop(farewell);
    };
}

/**
 * Closures vs. function pointers (fn)
 */

#[test]
fn non_capturing_closures_can_be_coerced_to_fn_pointers() {
    delim!();

    type Binop = fn(i32, i32) -> i32;

    let add = |x: i32, y: i32| x + y;
    println!("{:?}", add(3, 4));

    /**/

    let bo: Binop = add;
    println!("{:?}", bo(3, 4));
    delim!();
}

#[test]
fn capturing_closures_cannot_be_coerced_to_fn_pointers() {
    type Binop = fn(i32, i32) -> i32;

    let k = 10;
    let add = |x: i32, y: i32| x + y + k;

    // let bo: Binop = add;
    // println!("{}", bo(3, 4));
}

/**
 * No two closures, even if identical, have the same type.
 * No two function items, even if identical, have the same type, either.
 */

#[test]
fn no_two_closures_have_the_same_type() {
    delim!();

    fn compose<F>(x: i32, f: F, g: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        g(f(x))
    }

    // let result = compose(
    //     5,
    //     (|n: i32| n + n), // Change n + n to n * n and see what happens ...
    //     (|n: i32| n * n),
    // );
    // println!("{result}");

    // What if casting both to fn(i32) -> i32?
    let result = compose(
        5,
        (|n: i32| n + n) as fn(i32) -> i32,
        (|n: i32| n * n) as fn(i32) -> i32,
    );
    println!("{result}");

    delim!();
}

#[cfg(feature = "skip")]
#[test]
fn no_two_function_items_have_the_same_type() {
    fn compose<F>(x: i32, f: F, g: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        g(f(x))
    }

    fn double(x: i32) -> i32 {
        x + x
    }

    fn square(x: i32) -> i32 {
        x * x
    }

    let result = compose(5, double, square); // what if casting both to fn(i32) -> i32?
                                             // println!("{result}")
}

#[test]
fn solution_to_no_two_closures_have_the_same() {
    delim!();

    fn compose<F, G>(x: i32, f: F, g: G) -> i32
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
    {
        g(f(x))
    }

    fn double(x: i32) -> i32 {
        x + x
    }

    fn square(x: i32) -> i32 {
        x * x
    }

    let result = compose(5, |n: i32| n + n, |n: i32| n * n);
    println!("{result}");
    delim!();

    /**
     * A "function pointer" is kind of like a closure that has no environment.
     * As such, you can pass a function pointer to any function expecting a
     * closure argument, and it will work:
     */
    let result = compose(5, double, square);
    println!("{result}");
    delim!();
}

/**
 * Polymorphic types using trait object
 */

#[test]
fn polymorphic_types_using_trait_object() {
    delim!();

    type F = Box<dyn Fn(i32) -> i32>;
    fn compose(x: i32, f: F, g: F) -> i32 {
        g(f(x))
    }

    let result = compose(5, Box::new(|n: i32| n + n), Box::new(|n: i32| n * n));
    println!("{result}");
    delim!();

    fn double(x: i32) -> i32 {
        x * 2
    }

    fn square(x: i32) -> i32 {
        x * x
    }

    let result = compose(5, Box::new(double), Box::new(square));
    println!("{result}");

    delim!();
}
