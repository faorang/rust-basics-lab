use lib::delim;

/**
 * As output parameters:
 *
 * Closures as input parameters are possible, so returning closures as output
 * parameters should also be possible. However, anonymous closure types are,
 * by definition, unknown, so we have to use `impl Trait` to return them.
 * The valid traits for returning a closure are:
 *   - Fn
 *   - FnMut
 *   - FnOnce
 *
 * Beyond this, the `move` keyword must be used if need to capture local variables,
 * which signals that all captures occur **by value**. This is required because any
 * captures by reference would be dropped as soon as the function exited, leaving
 * invalid references in the closure.
 */

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is {text}!")
}

fn create_fnmut(text: &str) -> impl FnMut() {
    let mut text = text.to_owned();

    move || {
        text.push('!');
        println!("This is {}", text.to_uppercase());
    }
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || {
        println!("This is {text}!");
        std::mem::drop(text)
    }
}

#[test]
fn as_output_parameters_demo() {
    let fn_plain = create_fn();

    let text = String::from("FnMut");
    let mut fn_mut = create_fnmut(&text);

    let fn_once = create_fnonce();

    delim!();
    fn_plain();
    fn_mut();
    fn_once();
    delim!();
}

/**
 * `impl Trait` not allowed outside of function and inherent method return types.
 *  Not in trait definitions or any non-return type position.
 */
#[test]
fn abstract_return_type_nested_impl_not_allowed() {
    delim!();

    #[cfg(feature = "skip")]
    // Only concrete types are supported as nested `impl Trait` args
    fn make_print() -> impl Fn(impl ToString) {
        |text| println!("{}", text.to_string().to_uppercase())
    }

    fn make_print_ok<T>() -> impl Fn(T)
    where
        T: ToString,
    {
        |text| println!("{}", text.to_string().to_uppercase())
    }

    let f = make_print_ok();
    f("Rustopia!");

    delim!();
}

/**
 * Unlike argument position, different non-capturing closures can be used as
 * return values if they are of the same `fn` or closure type.
 *   - (kim: this applies only when returning closures)
 *
 * Non-capturing closures can be coerced to function pointers.
 * However, capturing closures cannot be coerced to function pointers.
 */
#[test]
fn returing_non_capturing_closures_with_the_same_type1() {
    delim!();

    fn returns_closure(flag: bool) -> impl Fn(i32, i32) -> i32 {
        match flag {
            true => |x, y| x + y,
            false => |x, y| x * y,
        }
    }

    let mut func1 = returns_closure(true);
    println!("{}", func1(3, 4));

    let mut func2 = returns_closure(false);
    println!("{}", func2(3, 4));

    let mut fp: &dyn Fn(i32, i32) -> i32 = &func1;
    println!("{}", fp(3, 4));

    fp = &func2;
    println!("{}", fp(3, 4));

    delim!();
}

#[test]
fn returing_non_capturing_closures_with_the_same_type2() {
    delim!();

    fn returns_closure(flag: bool) -> fn(i32, i32) -> i32 {
        if (flag) {
            |x, y| x + y
        } else {
            |x, y| x * y
        }
    }

    let mut func1 = returns_closure(true);
    println!("{}", func1(3, 4));

    let mut func2 = returns_closure(false);
    println!("{}", func2(3, 4));

    let mut fp: fn(i32, i32) -> i32 = func1;
    println!("{}", fp(3, 4));

    fp = func2;
    println!("{}", fp(3, 4));

    delim!();
}

#[cfg(feature = "skip")]
#[test]
fn returning_different_capturing_closures_not_allowed() {
    fn returns_closure(flag: bool, x: i32) -> impl Fn(i32) -> i32 {
        if (flag) {
            move |y| x + y
        } else {
            move |y| x * y
        }
    }

    let mut func1 = returns_closure(true, 3);
    println!("{}", func1(4));

    let mut func2 = returns_closure(false, 3);
    println!("{}", func2(4));
}
