use std::error;
use std::io;
use std::num;

pub fn run() {
    custom_error_demo1();
    // custom_error_demo2();
    // custom_error_demo3();
    // custom_error_demo4();
    // custom_error_demo5();
    // custom_error_demo6();
}

/**
* Method 1:
 * - Using `Result` and `String` as error type
 * - Using `match` to handle error
 */
fn custom_error_demo1() {
    fn do_it() -> Result<i32, String> {
        println!("Enter number: ");

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(e) => return Err(format!("Error: {e:?}")),
        }

        let number = match buffer.trim().parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(format!("Error: {e:?}")),
        };

        Ok(number)
    }

    match do_it() {
        Ok(n) => println!("You entered: {n}"),
        Err(e) => println!("Error: {e:?}"),
    }
}

/**
 * Method 2:
 * - Using `Result` and `enum` (io:Error, num::ParseIntError) as custom error type
 * - Using `match` to handle error
 */
enum MyError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

fn custom_error_demo2() {
    fn do_it() -> Result<i32, MyError> {
        println!("Enter number: ");

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(e) => return Err(MyError::IoError(e)),
        }

        let number = match buffer.trim().parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(MyError::ParseError(e)),
        };

        Ok(number)
    }

    match do_it() {
        Ok(n) => println!("You entered: {n}"),
        Err(e) => match e {
            MyError::IoError(e) => println!("Error: {e:?}"),
            MyError::ParseError(e) => println!("Error: {e:?}"),
        },
    }
}

/**
 * Method 3:
 * - Using `Result` and `enum` (io:Error, num::ParseIntError) as custom error type
 * - Using `?` to handle error
 * - Using `map_err` to handle error
 */
fn custom_error_demo3() {
    fn do_it() -> Result<i32, MyError> {
        println!("Enter number: ");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .map_err(MyError::IoError)?;

        let number = buffer.trim().parse::<i32>().map_err(MyError::ParseError)?;

        Ok(number)
    }

    match do_it() {
        Ok(n) => println!("You entered: {n}"),
        Err(e) => match e {
            MyError::IoError(e) => println!("Error: {e:?}"),
            MyError::ParseError(e) => println!("Error: {e:?}"),
        },
    }
}

/**
 * Method 4:
 * - Using `Result` and `enum` (io:Error, num::ParseIntError) as custom error type
 * - Using `?` to handle error
 * - Using `From` trait to convert `io::Error` to `MyError`
 * - Using `From` trait to convert `num::ParseIntError` to `MyError`
 */

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::IoError(e)
    }
}

impl From<num::ParseIntError> for MyError {
    fn from(e: num::ParseIntError) -> Self {
        MyError::ParseError(e)
    }
}

fn custom_error_demo4() {
    fn do_it() -> Result<i32, MyError> {
        println!("Enter number: ");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let number = buffer.trim().parse::<i32>()?;

        Ok(number)
    }

    match do_it() {
        Ok(n) => println!("You entered: {n}"),
        Err(e) => match e {
            MyError::IoError(e) => println!("Error: {e:?}"),
            MyError::ParseError(e) => println!("Error: {e:?}"),
        },
    }
}

/**
 * Method 5:
 * - Using `Result` and `Box<dyn error::Error>` as error type
 * - Using `?` to handle error
 */
fn custom_error_demo5() {
    fn do_it() -> Result<i32, Box<dyn error::Error>> {
        println!("Enter number: ");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let number = buffer.trim().parse::<i32>()?;

        Ok(number)
    }

    match do_it() {
        Ok(n) => println!("You entered: {n}"),
        Err(e) => println!("Error: {e:?}"),
    }
}

/**
 * Method 6:
 * - Using `Result` and `enum` (io:Error, num:ParseIntError) as custom error type
 * - Using `?` to handle error
 * - Using `thiserror` crate
 * - Using `#[from]` to convert `io::Error` to `MyError`
 * - Using `#[from]` to convert `num::ParseIntError` to `MyError`
 * - Using `#[error]` to format error message
 *
 */
fn custom_error_demo6() {
    use thiserror::Error;

    #[derive(Error, Debug)]
    enum MyError {
        #[error("Error: {0:?}")]
        IoError(#[from] io::Error),
        #[error("Error: {0:?}")]
        ParseError(#[from] num::ParseIntError),
    }

    fn do_it() -> Result<i32, MyError> {
        println!("Enter number: ");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let number = buffer.trim().parse::<i32>()?;

        Ok(number)
    }

    match do_it() {
        Ok(n) => println!("You entered: {n}"),
        Err(e) => println!("Error: {e:?}"),
    }
}
