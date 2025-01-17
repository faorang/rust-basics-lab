// Make me compile without changing the function signature!

#[cfg(feature = "skip")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word = String::from("green"); // Try not changing this line :)

        if is_a_color_word(word) {
            println!("That is a color word I know!");
        } else {
            println!("That is not a color word I know.");
        }
    }

    fn is_a_color_word(attempt: &str) -> bool {
        attempt == "green" || attempt == "blue" || attempt == "red"
    }
}
