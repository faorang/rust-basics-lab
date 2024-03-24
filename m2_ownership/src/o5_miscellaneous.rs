
#[cfg(feature = "skip")]
#[test]
fn move_when_pattern_matching() {
    let name: Option<String> = Some("Leto".to_string());

    // if name.is_none() {
    //     println!("no name");
    // } else {
    //     println!("we have a name: {}", name.unwrap());
    // }

    match name {
        None => println!("no name"),
        Some(name) => println!("we have a name: {}", name),
    }
    println!("{:?}", name)
}

/*
 * The compiler will tell us that our last line is invalid because `name`
 * has been partially moved. Note that `name` is an Option<String>, and we
 * can think of the actual string value, "Leto", as a nested field.
 * The second arm of our match takes ownership of the the string value.
 * The result is that name, the Option<String>, is no longer valid since
 * part of its data has been moved. We can solve this by matching against
 * a borrow:
 */
#[test]
fn mstching_against_borrow() {
    let name: Option<String> = Some("Leto".to_string());

    match &name {
        None => println!("no name"),
        Some(name) => println!("we have a name: {}", name),
    }
    println!("{:?}", name)
}

#[test]
fn mstching_against_borrow_using_ref() {
    let name: Option<String> = Some("Leto".to_string());

    match name {
        None => println!("no name"),
        Some(ref name) => println!("we have a name: {}", name),
    }
    println!("{:?}", name)
}

/*
 * It's worth remembering that in addition to moving and borrowing,
 * there's also copying. If we replace our Option<String> with an
 * Option<T> where T implements copying, our initial borrow-free
 * version works at the "cost" of copying the value:
 */
#[test]
fn copying_when_pattern_matching() {
    let id: Option<&str> = Some("id007");

    match id {
        None => println!("no id"),
        Some(id) => println!("we have a id: {id}"),
    }
    println!("{id:?}")
}
