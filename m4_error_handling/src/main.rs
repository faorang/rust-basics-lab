#![allow(unused)]

mod e2_propagatoin;
mod e3_custom_error;

use lib::delim;
use std::env::current_dir;

fn main() {
    delim!();

    // propagation_demo();
    custom_error_demo();

    delim!();
}

fn propagation_demo() {
    e2_propagatoin::run();
}

fn custom_error_demo() {
    e3_custom_error::run();
}
