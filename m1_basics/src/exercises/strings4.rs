// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

#[cfg(feature = "skip")]
#[test]
fn test() {
    ______("blue");
    ______("red".to_string());
    ______(String::from("hi"));
    ______("rust is fun!".to_owned());
    ______("nice weather".into());
    ______(format!("Interpolation {}", "Station"));
    ______(&String::from("abc")[0..1]);
    ______("  hello there ".trim());
    ______("Happy Monday!".to_string().replace("Mon", "Tues"));
    ______("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
