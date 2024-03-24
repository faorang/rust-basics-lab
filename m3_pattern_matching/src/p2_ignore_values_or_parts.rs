/**
 * Ignoring Values in a Pattern:
 *
 * There are a few ways to ignore entire values or parts of values in a pattern:
 * - using the _ pattern (which you’ve seen),
 * - using the _ pattern within another pattern,
 * - using a name that starts with an underscore, or
 * - using .. to ignore remaining parts of a value.
 */

/**
 * Ignoring an Entire Value with _
 */
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

#[test]
fn ignore_values_in_formal_parameters() {
    foo(3, 4); // first parameter will be ignored
}

/**
 * Ignoring Parts of a Value with Nested _
 */
#[test]
fn ignore_parts_of_a_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    /**/

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

/**
 * Ignoring an Unused Variable by Starting Its Name with _
 */
#[test]
fn declaring_unused_variables() {
    let _x = 5; // No compiler warning!
    let y = 10;

    println!("y = {y}");
}

#[test]
fn ignore_unused_but_bound_matches() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s); // s is still moved into the `if let` expression
}

#[test]
fn ignore_unused_matches_without_binding() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}"); // s can be used again becasue no binding was created
}

/**
 * Ignoring Remaining Parts of a Value with `..`
 */
#[test]
fn ignore_remaining_parts() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    /**/

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
