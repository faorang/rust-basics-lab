use lib::delim;

/**
 * Higher Order Functions:
 *
 * Rust provides Higher Order Functions (HOF). These are functions that take one or
 * more functions and/or produce a more useful function. HOFs and lazy iterators give
 * Rust its functional flavor.
 */

fn is_odd(n: &i32) -> bool {
    n % 2 != 0
}

fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

fn is_all(_: &i32) -> bool {
    true
}

mod hof_demo1 {
    use super::*;

    fn total(vs: &[i32]) -> i32 {
        let mut sum = 0;

        for v in vs {
            sum += v;
        }
        sum
    }

    #[test]
    fn functions_with_dupicated_logic() {
        let vs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        delim!();
        println!("total: {}", total(&vs));
        // println!("even total: {}", total_even(&vs));
        // println!("odd total: {}", total_odd(&vs));
        delim!();
    }
}

mod hof_demo2 {
    use super::*;

    /**
     * Define a higher order function `total()` that takes a function as an argument.
     */
    fn total(vs: &[i32], predicate: ()) -> i32 {
        todo!()
    }

    #[test]
    fn higher_order_functions() {
        let vs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        /*
         * Function items can be coerced to function pointers.
         */
        delim!();
        // println!("total: {}", total(&vs, is_all));
        // println!("even total: {}", total(&vs, is_even));
        // println!("odd total: {}", total(&vs, is_odd));
        delim!();
    }
}

mod hof_demo3 {
    fn total(vs: &[i32], predicate: fn(&i32) -> bool) -> i32 {
        let mut sum = 0;

        for v in vs {
            if (predicate(v)) {
                sum += v;
            }
        }
        sum
    }
    // is equivalent to
    fn filter_sum(vs: &[i32], predicate: fn(&i32) -> bool) -> i32 {
        vs.iter().filter(|&v| predicate(v)).sum()
    }

    #[test]
    fn passing_closures_to_hofs() {
        use super::*;

        let vs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        /*
         * Non-capturing closures can be coerced to function pointers
         */
        delim!();
        println!("total: {}", total(&vs, |_| true));
        println!("even total: {}", total(&vs, |n| n % 2 == 0));
        println!("odd total: {}", total(&vs, |n| n % 2 != 0));
        delim!();
    }

    #[test]
    fn types_of_closures() {
        /*
         * Non-capturing closures can be coerced to function pointers
         */
        let f = |x: i32| {
            println!("{}", x.pow(2));
        };

        /*
         * Capturing closures cannot be coerced to function pointers
         */
        let count = 42;
        let f = || {
            println!("{count}");
        };

        let mut count = 42;
        let f = || {
            count += 1;
            count
        };

        let s = String::from("hello");
        let f = || {
            let c = s;
            c.len()
        };
    }

    #[test]
    fn capturing_closures_cannot_be_coerced_to_function_pointer() {
        fn foo(f: fn() -> usize) {
            f();
        }

        let count = 42;
        let f = || count; // "fn() -> i32" is a shape, not type!

        /*
         * Capturing closures cannot be coerced to function pointers
         */
        // foo(f);
    }
}

mod hof_demo4 {
    use super::*;

    fn filter_sum(vs: &[i32], predicate: impl Fn(&i32) -> bool) -> i32 {
        vs.iter().filter(|&v| predicate(v)).sum()
    }

    #[test]
    fn test() {
        let vs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        /*
         * Non-capturing and/or capturing closures can be coerced to function traits
         */
        delim!();
        println!("total: {}", filter_sum(&vs, |_| true));
        println!("even total: {}", filter_sum(&vs, |n| n % 2 == 0));
        println!("odd total: {}", filter_sum(&vs, |n| n % 2 != 0));
        delim!();
        /*
         * Function items can be coerced to function traits.
         */
        println!("total: {}", filter_sum(&vs, is_all));
        println!("even total: {}", filter_sum(&vs, is_even));
        println!("odd total: {}", filter_sum(&vs, is_odd));
        delim!();
    }
}

mod higher_order_functions_general_usage {
    use super::*;

    fn is_odd(n: u32) -> bool {
        n % 2 != 0
    }

    #[test]
    fn imperative_style() {
        delim!();
        println!("Find the sum of all the squared odd numbers under 1000");
        const UPPER_BOUND: u32 = 1000;

        // Declare accumulator variable
        let mut acc = 0;
        // Iterate: 0, 1, 2, ... to infinity
        for n in 0.. {
            // Square the number
            let n_squared = n * n;

            if n_squared >= UPPER_BOUND {
                // Break loop if exceeded the upper limit
                break;
            } else if is_odd(n_squared) {
                // Accumulate value, if it's odd
                acc += n_squared;
            }
        }

        println!("imperative style: {acc}");
        delim!();
    }

    #[test]
    fn functional_style() {
        delim!();
        println!("Find the sum of all the squared odd numbers under 1000");
        const UPPER_BOUND: u32 = 1000;

        let result = (0..)
            .map(|n| n * n)
            .filter(|&n_squared| is_odd(n_squared))
            .take_while(|&n_squared| n_squared < UPPER_BOUND);

        println!("{:?}", result.clone().collect::<Vec<u32>>());

        let sum_of_squared_odd_numbers: u32 = result.sum();
        println!("functional style: {sum_of_squared_odd_numbers}");
        delim!();
    }
}

/*
 * HOF Use Cases
 */

#[test]
fn use_case_filter() {
    let a = [0i32, 1, 2];
    let mut iter = a.iter().filter(|&x| x.is_positive());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}

#[test]
fn use_case_map() {
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| 2 * x);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);
}

#[test]
fn use_case_option_handling() {
    let maybe_some_string = Some(String::from("Hello, World!"));

    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));
}

#[test]
fn use_case_callback() {
    struct Button {
        label: String,
        callback: Option<fn(&mut Button)>,
    }

    impl Button {
        fn new_with_label(label: &str) -> Button {
            Button {
                label: label.to_owned(),
                callback: None,
            }
        }

        fn set_label(&mut self, label: &str) {
            self.label = label.to_owned();
        }

        fn register(&mut self, callback: fn(&mut Button)) {
            self.callback = Some(callback);
        }

        fn click(&mut self) {
            if let Some(callback) = self.callback {
                callback(self);
            }
        }
    }

    delim!();
    let mut button = Button::new_with_label("Click me!");
    println!("button label: {}", button.label);

    button.register(|but: &mut Button| {
        but.set_label("I've been clicked!");
    });

    button.click();
    println!("button label: {}", button.label);
    delim!();
}

#[test]
fn use_case_decorator() {
    fn triple(x: i32) -> i32 {
        x * 3
    }

    fn triple_decor(f: fn(i32) -> i32) -> impl Fn(i32) -> i32 {
        move |x| {
            println!("triple will be called");
            f(x)
        }
    }

    delim!();
    let triple = triple_decor(triple);

    println!("{}", triple(3));
    println!("{}", triple(7));
    delim!();
}
