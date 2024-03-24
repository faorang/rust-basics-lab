#[test]
fn tuple_basics() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
}

#[test]
fn fibonacci_demo() {
    fn fibo(n: i32) -> i32 {
        if n == 0 || n == 1 {
            n
        } else {
            fibo(n - 1) + fibo(n - 2)
        }
    }

    for i in 0..=42 {
        println!("fibo({}) = {}", i, fibo(i));
    }
}

#[test]
fn fast_fibonacci_demo() {
    fn fibo(n: i32) -> (i32, i32) {
        if n == 0 || n == 1 {
            (n, 0)
        } else {
            let (current, previous) = fibo(n - 1);
            (current + previous, current)
        }
    }

    for i in 0..=45 {
        println!("fibo({}) = {}", i, fibo(i).0);
    }
}

/**
 * A fixed-size array, denoted [T; N], for the element type, T, and
 * the non-negative compile-time constant size, N.
 *
 * - A list with each element, i.e., [x, y, z].
 * - A repeat expression [x; N], which produces an array with N copies of x.
 *   The type of x must be `Copy`.
 *
 * Arrays coerce to slices ([T]), so a slice method may be called on an array.
 *
 * Slices have a dynamic size and do not coerce to arrays. Instead, use
 * slice.try_into().unwrap() or <ArrayType>::try_from(slice).unwrap().
 */

#[test]
fn array_creation() {
    let mut array: [i32; 3] = [0; 3];
    println!("{:?}", array);

    let array = [1, 2, 3, 4, 5];
    assert_eq!(array[0], 1);
    assert_eq!(array[array.len() - 1], 5);
    assert_eq!(array[1..], [2, 3, 4, 5]);

    for x in array {
        print!("{x} ");
    }
    println!();

    // Iterate over reference to the array’s elements
    for x in &array {
        print!("{x} ");
    }
    println!();
}

#[test]
fn array_creation_using_try_from() {
    /**
     * You can use `<ArrayType>::try_from(slice)` or `slice.try_into()` to get an array from a slice:
     */
    let array: [u8; 3] = [1, 0, 2];

    let array2 = <[u8; 2]>::try_from(&array[1..3]).unwrap();
    assert_eq!(array2, [0, 2]);

    let array2: [u8; 2] = array[1..3].try_into().unwrap();
    assert_eq!(array2, [0, 2]);

    let slice = &array[1..3];
    assert_eq!(slice, [0, 2]); // array coerce to slice
}

#[test]
fn array_as_slice() {
    let array = [1, 2, 3];
    let slice = array.as_slice();
    assert_eq!(array, slice); // array coerce to slice

    let array = [1, 2, 3];
    let slice = array.as_ref();
    assert_eq!(array, slice);

    let array = [1, 2, 3];
    let slice = &array[..];
    assert_eq!(array, slice);

    let mut array = [1, 2, 3];
    let slice = array.as_mut_slice();
    slice[0] = 42;
    assert_eq!([42, 2, 3], slice);
}
