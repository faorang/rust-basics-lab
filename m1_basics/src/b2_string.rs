#![allow(non_snake_case)]

use lib::delim;
use unicode_segmentation::UnicodeSegmentation;

#[test]
fn strings_are_stored_as_a_collection_of_UTF8_encoded_bytes() {
    let s1 = String::new();
    let s2 = "Hello";
    let s3 = s2.to_string();
    let s4 = String::from("Hello, world");
    println!("{s1}, {s2}, {s3}, {s4}");
}

#[test]
fn strings_are_stored_as_a_collection_of_UTF8_encoded_bytes2() {
    let hello1 = String::from("안녕하세요");
    let hello2 = String::from("नमस्ते");
    let hello3 = String::from("こんにちは");
    let hello4 = String::from("你好");
    let hello5 = String::from("Привет");
    let hello6 = String::from("Olá");

    println!("{hello1}, {hello2}, {hello3}, {hello4}, {hello5}, {hello6}");
}

#[test]
fn concatenation() {
    let hello = "Hello, ".to_string();
    let world = "World".to_string();

    let hello_world = hello + &world;
    println!("{hello_world}");

    // println!("{hello}"); // borrow after move error!
    println!("{world}");
}

#[test]
fn ops() {
    let mut string = String::from("Hello, ");
    string.push_str("World");
    string.push('!');
    println!("{string}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = format!("{s1}{s2}");
    println!("{s3}");
}

#[rustfmt::skip]
#[test]
fn string_in_UTF8() {
    let hello1 = "안녕하세요"; // 15 bytes
    let hello2 = "नमस्ते"; // Hindi, 18 bytes
    // Bytes
    // [236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148]
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    // Unicode Scala Code
    // ['안', '녕', '하', '세', '요']
    // ['न', 'म', 'स', '्', 'त', 'े']

    // Graphemes clusters
    // ["안", "녕", "하", "세", "요"]
    // ["न", "म", "स्", "ते"]

    delim!();
    println!("{:?}", hello1.bytes().collect::<Vec<u8>>());
    delim!();
    
    println!("{:?}", hello1.chars().collect::<Vec<char>>());
    delim!();
    
    println!("{:?}", hello1.graphemes(true).collect::<Vec<&str>>());
    delim!();

    println!("{:?}", hello2.bytes().collect::<Vec<u8>>());
    delim!();

    for ch in hello2.chars() {
        println!("{ch}");
    }
    delim!();

    for g in hello2.graphemes(true) {
        println!("{g}");
    }
    delim!()
}

#[test]
fn accessing_character() {
    let hello = "안녕하세요";
    // let hello = "नमस्ते"; // Hindi

    // let ch = hello[0]; // only allowed by indexing using ranges

    let ch = hello.chars().nth(3);
    match ch {
        Some(c) => println!("{c}"),
        None => println!("No character found"),
    }

    let ch = hello.graphemes(true).nth(3);
    match ch {
        Some(c) => println!("{c}"),
        None => println!("No character found"),
    }

    // ranges must start at a character boundary
    let ch = &hello[3..]; // Change to 4 and see what happens
    dbg!(ch);
}
