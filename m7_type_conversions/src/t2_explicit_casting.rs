//!
//! # Explicit casting using `as` keyword
//!

#[test]
fn casting_between_numerics() {
    let x: u32 = 9;
    let y = x as u64; // widening
    let z: u64 = x.into();

    let x: u32 = std::u32::MAX; // 4294967295
    let y = x as u16; // 65535
                      // let y: u16 = x.into(); // From(u32) for u16 not implemented
}

#[test]
fn some_casting_examples_between_allowed_pairs() {
    let one = true as u8;
    println!("{one}"); // 1

    let at_sign = 64 as char;
    println!("{at_sign}"); // @

    let two_hundred = -56i8 as u8;
    println!("{two_hundred}"); // 200
}

#[test]
fn casting_between_raw_pointers_and_integers() {
    let a = 300 as *const char; // `a` is a pointer to location 300.
    let a = 300 as *mut char; // `a` is a pointer to location 300.

    let b = a as u32;
}

#[test]
fn casting_between_references_and_raw_pointers_and_integers() {
    let x = 42;
    println!("{x}"); // 42
    let y = &x;
    println!("&y = {:p}, y = {}", y, y);

    let z = y as *const i32 as usize;
    println!("{z}");

    let y = z as *const i32;
    println!("{:p}", y);
    println!("{}", unsafe { *y });
}

#[test]
fn casting_c_like_enums_to_numerics() {
    enum Fruit {
        Apple = 10,
        Banana,
        Orange,
    }

    let x = Fruit::Apple as u16;
    let x = Fruit::Apple as u32;
    let x = Fruit::Apple as i32;

    // let x = Fruit::Apple as f64;
}
