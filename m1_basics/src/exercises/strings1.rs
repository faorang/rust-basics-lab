// Make me compile without changing the function signature!

#[cfg(feature = "skip")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let answer = current_favorite_color();
        println!("My current favorite color is {answer}");
    }

    fn current_favorite_color() -> String {
        "blue"
    }
}
