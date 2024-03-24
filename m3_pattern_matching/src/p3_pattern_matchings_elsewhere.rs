/**
 * Conditional `if let` Expressions:
 *
 * Use `if let` expressions mainly as a shorter way to write the equivalent of a match
 * that only matches one case. Optionally, `if let` can have a corresponding `else`
 * containing code to run if the pattern in the `if let` doesn’t match.
 *
 * Rust doesn't require that the conditions in a series of `if let`, `else if`,
 * `else if let` arms relate to each other.
 *
 * The downside of using `if let` expressions is that the compiler doesn’t check for
 * exhaustiveness, whereas with `match` expressions it does.
 */
#[test]
fn matching_ing_if_let_conditional() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/**
 * `while let` Conditional Loops:
 */
#[test]
fn matching_in_while_let_loop() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

/**
 * `for` Loops:
 *
 * In a `for` loop, the value that directly follows the keyword `for` is a pattern.
 * For example, in `for x in y` the `x` is the pattern.
 */
#[test]
fn matching_in_for_loop() {
    let vs = vec!['a', 'b', 'c'];

    for (index, value) in vs.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

/**
 * `let` Statements:
 *
 * let PATTERN = EXPRESSION;
 */
#[test]
fn let_binding_uses_pattern_matching() {
    let x = 5; // “bind everything to the variable x, whatever the value is.”

    let (x, y, z) = (1, 2, 3);
    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");

    // let (x, y) = (1, 2, 3); // error: expected tuple with 2 elements, found tuple with 3 elements
    let (x, y, _) = (1, 2, 3);
}
