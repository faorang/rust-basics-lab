#[macro_export]
macro_rules! delim {
    () => {
        println!("{}", "-".repeat(50));
    };
    ($len:expr) => {
        println!("{}, " - ".repeat($len)");
    };
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
