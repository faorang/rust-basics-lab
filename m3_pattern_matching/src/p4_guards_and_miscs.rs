/**
 * Extra Conditionals with Match Guards:
 *
 * A "match guard" is an additional if condition, specified after the pattern
 * in a match arm, that must also match for that arm to be chosen. Match guards
 * are useful for expressing more complex ideas than a pattern alone allows.
 *
 * The downside of this additional expressiveness is that the compiler doesn't
 * try to check for exhaustiveness when match guard expressions are involved.
 */
#[test]
fn conditional_with_match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
}

// Using a match guard to test for equality with an outer variable.
#[test]
fn matching_using_outer_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

/**
 * You can also use the or operator | in a match guard to specify multiple patterns;
 * the match guard condition will apply to all the patterns.
 */
#[test]
fn guard_applied_to_multiple_patterns() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

/**
 * The @ Bindings:
 *
 * The at operator `@` lets us create a variable that holds a value at the same time
 * as we’re testing that value for a pattern match.
 */
#[test]
fn ceate_variable_holding_matched_value_using_at_binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

/**
 * Function Parameters:
 *
 * Function parameters can also be patterns.
 */

#[test]
fn pattern_matching_for_parameters() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
