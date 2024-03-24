use std::env::current_dir;

use std::net::IpAddr;
use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
};

#[test]
#[should_panic(expected = "Crash and burn")]
fn uncondition_fail_with_panic() {
    panic!("Crash and burn");
}

#[test]
fn manual_handling_with_match() {
    let dir = current_dir().unwrap();
    println!("Current dir: {:?}", dir);

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

#[test]
fn unwrap() {
    let f = File::open("hello.txt").unwrap();
    println!("f: {:?}", f);

    let home: IpAddr = "127.0.0.0".parse().unwrap();
    println!("home: {:?}", home);
}

/**
 * We recommend that `expect` messages are used to describe the problem.
 */
#[test]
fn describe_reason_with_expect() {
    let f = File::open("hello.txt").expect("Fail to open hello.txt");
}

#[test]
pub fn manual_handling_with_nested_match() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

#[test]
pub fn manual_handling_with_unwrap_or_else() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
