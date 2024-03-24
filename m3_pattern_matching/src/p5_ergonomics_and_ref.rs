/**
 * RFC 2005 (a.k.a. match ergonomics) introduced the rule.
 *
 * Before the change was implemented, there were 2 ways to write this match.
 *
 * 1. Match on `self` and prefix each pattern with `&` to "destructure" the reference.
 *
 * ```
 * # enum List {
 * #     Cons(i32, Box<List>),
 * #     Nil,
 * # }
 * # use List::*;
 * # impl List {
 * fn tail(&self) -> Option<&List> {
 *     match self {
 *         &Cons(_, ref item) => Some(item),
 *         &Nil => None,
 *     }
 * }
 * # }
 * ```
 *
 * 2. Match on `*self` and don't prefix each pattern with `&` (because `*self` is not a reference).
 * ```
 * # enum List {
 * #     Cons(i32, Box<List>),
 * #     Nil,
 * # }
 * # use List::*;
 * # impl List {
 * fn tail(&self) -> Option<&List> {
 *     match *self {
 *         Cons(_, ref item) => Some(item),
 *         Nil => None,
 *     }
 * }
 * # }
 * ```
 *
 * Yet, in both cases, we need to write `ref item`, otherwise we'll get error[E0507]:
 * cannot move out of borrowed content.
 *
 * However, in the match you've written, the expression being matched is a reference
 * (type &List) but the patterns are not reference patterns (as in 1. above). This is
 * where match ergonomics kick in: the rule says that "when a reference is matched with
 * a non-reference pattern", the bindings within that pattern "bind by reference" rather
 * than "by value" (i.e. as if they were prefixed with `ref`).
 */

#[test]
fn test_case1() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::*;

    impl List {
        fn tail(&self) -> Option<&List> {
            match self {
                &Cons(_, ref item) => Some(item),
                &Nil => None,
            }
        }
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    let tail = list.tail();
    println!("tail = {:?}", tail);
}

#[test]
fn test_case2() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::*;

    impl List {
        fn tail(&self) -> Option<&List> {
            match *self {
                Cons(_, ref item) => Some(item),
                Nil => None,
            }
        }
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    let tail = list.tail();
    println!("tail = {:?}", tail);
}

#[test]
fn test_ergonomics() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::*;

    impl List {
        fn tail(&self) -> Option<&List> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    let tail = list.tail();
    println!("tail = {:?}", tail);
}

#[test]
fn another_demo() {
    #[derive(Debug)]
    struct Foo {
        x: String,
    }

    let a = Foo {
        x: "42".to_string(),
    };
    let a_ref = &a;

    // Case 1
    match &a_ref {
        &Foo { ref x } => println!("x = {}", x), // no move occurs
        _ => println!("x != 42"),
    }
    println!("a = {:?}", a);

    // Case 2
    match *a_ref {
        Foo { ref x } => println!("x = {}", x), // no move occurs
        _ => println!("x != 42"),
    }
    println!("a = {:?}", a);

    // Case 3
    match a_ref {
        Foo { x } => println!("x = {}", x), // no move occurs
        _ => println!("x != 42"),
    }
    println!("a = {:?}", a);
}

/**
 * `ref` pattern
 */
#[test]
fn top_level_ref_let_pattern_is_discouraged() {
    let x = "42".to_string();
    let ref y = x; // y is bound to x by reference, i.e., no move occurs

    println!("x = {}", x);
    println!("y = {}", *y);
}

#[test]
fn take_a_reference_with_ampersand_instead_of_top_level_ref_in_let_pattern() {
    let x = "42".to_string();
    let y = &x;

    println!("x = {}", x);
    println!("y = {}", *y);
}

#[test]
fn top_level_ref_arg_ignored() {
    fn foo(ref x: String) {
        println!("[foo] x = {}", *x);
    }

    let x = "42".to_string();
    foo(x); // x is moved into foo, oops!

    // println!("[main] x = {}", x);
}

#[test]
fn using_a_reference_type_instead_of_top_level_ref_arg() {
    fn foo(x: &String) {
        println!("[foo] x = {}", *x);
    }

    let x = "42".to_string();
    foo(&x);

    println!("[main] x = {}", x);
}
