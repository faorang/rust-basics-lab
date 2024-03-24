#![allow(unused)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_owned(),
            age,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub trait ToJson {
    fn to_json(&self) -> String;
}

/*
 * TODO: Implement `ToJson` trait for `Person` struct
 *      using `serde_json::to_string` function.
 */

/*
 * TODO: Implement `ToJson` trait for `Coord` struct.
 *      using `serde_json::to_string` function.
 */

/*
 * TODO: Implement `ToJson` trait for `&T` and `&mut T` as Blanket implementations.
 */

// impl<T> ToJson for &T
// where
//     T: Serialize,
// {
//     fn to_json(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
// }

// impl<T> ToJson for &mut T
// where
//     T: Serialize,
// {
//     fn to_json(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
// }
