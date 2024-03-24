use std::{
    env::current_dir,
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

pub fn run() {
    let dir = current_dir().unwrap();
    println!("Current dir: {:?}", dir);

    match read_username_from_file("hello.txt") {
        Ok(s) => println!("User name: {s}"),
        Err(e) => println!("Error: {:?}", e),
    };
}

/**
 * Error Propagation
 */

// type io::Result<T> = Result<T, io::Error>
// Result<File, Error> => Result<String, Error>
// #[cfg(feature = "skip")]
pub fn read_username_from_file<P>(file_name: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let f = File::open(file_name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

#[cfg(feature = "skip")]
pub fn read_username_from_file<P>(file_name: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    // if error, returns error from current function for caller to handle
    let mut f = File::open(file_name)?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

#[cfg(feature = "skip")]
pub fn read_username_from_file<P>(file_name: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut s = String::new();
    let f = File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(feature = "skip")]
pub fn read_username_from_file<P>(file_name: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    // This is a convenience function for using `File::open` and `read_to_string`
    // with fewer imports and without an intermediate variable.
    fs::read_to_string(file_name)
}
