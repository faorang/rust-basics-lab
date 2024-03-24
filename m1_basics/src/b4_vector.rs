/**
 * Vector: a contiguous growable array type, written as Vec<T>, short for ‘vector’.
 */

#[test]
fn vector_allocation() {
    let v: Vec<i32> = Vec::new();

    assert_eq!(v.capacity(), 0);
    assert_eq!(v.len(), 0);
    assert!(v.is_empty());

    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() >= 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert!(vec.capacity() >= 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    // A vector of a zero-sized type will always over-allocate, since no
    // allocation is necessary
    let vec_units = Vec::<()>::with_capacity(10);
    assert_eq!(vec_units.capacity(), usize::MAX);
}

#[test]
fn creation_from_array_using_from() {
    let array = [1, 2, 3, 4];
    let mut vs = Vec::from(array);
    assert_eq!(vs, [1, 2, 3, 4]);

    // Here, `array` and `vs` can be modified independently.
    vs[0] = 42;
    println!("array: {:?}, vs: {:?}", array, vs);
}

#[test]
fn creation_from_array_using_to_vec() {
    let array = [10, 20, 30];
    let mut vs = array.to_vec();

    // Here, `array` and `vs` can be modified independently.
    vs[0] = 42;
    println!("array: {:?}, vs: {:?}", array, vs);

    let strings = "hello, world";
    let vs = strings.chars().collect::<Vec<_>>();
    dbg!(vs);
}

#[test]
fn creation_using_vec_macro() {
    let vs = vec![1, 2, 3, 4];
    assert_eq!(vs, [1, 2, 3, 4]);

    let vs = vec![0; 4];
    assert_eq!(vs, [0, 0, 0, 0]);

    // The following is equivalent, but potentially slower:
    let mut vs = Vec::with_capacity(0);
    vs.resize(4, 0);
    assert_eq!(vs, [0, 0, 0, 0]);
}

#[test]
fn clear_vector() {
    let mut v = vec![1, 2, 3];
    dbg!(v.capacity());
    dbg!(v.len());

    v.clear();

    assert!(v.is_empty());
    dbg!(v.capacity());
    dbg!(v.len());
}

#[test]
fn get_slice_from_vector() {
    /**
     * To get a slice, use &.
     */
    fn read_slice(slice: &[usize]) {
        // ...
    }

    fn read_slice_mut(slice: &mut [usize]) {
        slice[0] = 42;
    }

    let vs = vec![0, 1];
    read_slice(&vs);

    // ... and that's all!
    // you can also do it like this:
    let u: &[usize] = &vs;
    // or like this:
    let u: &[_] = &vs;

    /**/

    let mut vs = vec![0, 1];
    read_slice_mut(&mut vs);
    println!("{:?}", vs);
}

#[test]
#[should_panic]
fn vector_indexing() {
    let v = vec![0, 2, 4, 6];
    assert_eq!(v[1], 2);

    // panic!
    dbg!(v[6]);
}

#[test]
fn get_element_from_vector() {
    let v = vec![10, 40, 30];

    let first = v.get(1);
    assert_eq!(v.get(1), Some(&40));
    assert_eq!(v.get(3), None);
    assert_eq!(v.get(0..2), Some(&[10, 40][..]));
    assert_eq!(v.get(0..4), None);
}

#[test]
fn test_push_pop() {
    let mut v = vec![];
    v.push(3);
    v.push(4);
    assert_eq!(v.pop(), Some(4));
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), None);
}

#[test]
fn test_reverse() {
    let mut v = [1, 2, 3];
    v.reverse();
    assert!(v == [3, 2, 1]);
}

#[test]
fn test_split_at() {
    let v = [1, 2, 3, 4, 5, 6];

    let (left, right) = v.split_at(0);
    assert_eq!(left, []);
    assert_eq!(right, [1, 2, 3, 4, 5, 6]);

    let (left, right) = v.split_at(2);
    assert_eq!(left, [1, 2]);
    assert_eq!(right, [3, 4, 5, 6]);

    let (left, right) = v.split_at(6);
    assert_eq!(left, [1, 2, 3, 4, 5, 6]);
    assert_eq!(right, []);
}
