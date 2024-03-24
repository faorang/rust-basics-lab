/**
 * Curried functions are functions that take one argument and return another function
 * that takes the next argument and so on until all arguments are consumed. The last
 * function returns the result of the function call.
 *
 * λ x y . (x + y) => λ x . λ y . (x + y)
 */

/**
 * `impl Trait` can only appear as a parameter or return type of free or inherent methods.
 */

#[test]
fn test() {
    // Normal function
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Curried version of `add` function.
    let add_curried = |x| move |y| x + y;

    // Define `inc` as a closure that increments its argument by 1.
    let inc = add_curried(1);

    assert_eq!(inc(2), 3);
}

#[test]
fn currency_converters() {
    fn convert_to_usd(exchange_rate: f64, amount: f64) -> f64 {
        amount * exchange_rate
    }

    fn cvt_to_usd(exchange_rate: f64) -> impl (Fn(f64) -> f64) {
        move |amount: f64| amount * exchange_rate
    }

    let won_to_usd = cvt_to_usd(0.00083);

    println!("{}", won_to_usd(100_000.0));
}

#[test]
fn currency_converters2() {
    fn convert_to_usd(base_currency: String, exchange_rate: f64) -> impl (Fn(f64) -> f64) {
        move |amount| {
            println!("Converting {amount} {base_currency} to USD");
            amount * exchange_rate
        }
    }

    let won_to_usd = convert_to_usd(String::from("KRW"), 0.00083);
    let yen_to_usd = convert_to_usd(String::from("JPY"), 0.0094);

    println!("{}", won_to_usd(100_000.0));
    println!("{}", yen_to_usd(100_000.0));
}

#[test]
fn currency_converters3() {
    let convert_to_usd = todo!();

    //     let won_to_usd = convert_to_usd(String::from("KRW"))(0.00083);
    //     let yen_to_usd = convert_to_usd(String::from("JPY"))(0.0094);
    //
    //     println!("{}", won_to_usd(100_000.0));
    //     println!("{}", won_to_usd(200_000.0));
    //     println!("{}", yen_to_usd(100_000.0));
}
