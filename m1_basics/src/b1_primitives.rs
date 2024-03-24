/**
 * A char is a ‘Unicode scalar value’, which is any ‘Unicode code point’ [0..0x10FFFF] other
 * than a surrogate code point. Surrogate code points, used by UTF-16, are in [0xD800..0xDFFF].
 *
 * `char` is always 4 bytes in size. This is a different representation than a given
 * character would have as part of a `String`.
 */

#[test]
fn char_demo() {
    let v = vec!['h', 'e', 'l', 'l', 'o'];

    // five elements times four bytes for each element
    assert_eq!(20, v.len() * std::mem::size_of::<char>()); // turbo fish `::<>`
    assert_eq!(24, std::mem::size_of_val(&v)); // [pointer, capacity, length]
    assert_eq!(20, std::mem::size_of_val(&v[..]));
}

#[test]
fn string_demo() {
    let s = String::from("hello");

    // five elements times one byte per element
    assert_eq!(5, s.len() * std::mem::size_of::<u8>());
    assert_eq!(24, std::mem::size_of_val(&s)); // [pointer, capacity, length]
    assert_eq!(5, std::mem::size_of_val(s.as_str()));
}

#[test]
fn processing_by_char_may_consume_more_memory() {
    let s = String::from("love: ❤️");
    println!("{} u8's", s.len());
    assert_eq!(12, std::mem::size_of_val(&s[..]));

    let v: Vec<char> = s.chars().collect();
    println!("{} chars", v.len());
    assert_eq!(32, std::mem::size_of_val(&v[..]));
}

/**
 * `str` is a string slice, and is the most primitive string type.
 */

#[test]
fn test_str() {
    let hello_world = "Hello, World!";
}
