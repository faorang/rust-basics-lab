use std::collections::HashMap;

#[test]
fn create_empty_map() {
    let mut map: HashMap<String, u32> = HashMap::new();
}

#[test]
fn from_array() {
    let mut map = HashMap::from([("one", 1), ("two", 2)]);

    let map2: HashMap<&str, u32> = [("one", 1), ("two", 2)].into();

    let map3: HashMap<&str, u32> = [("one", 1), ("two", 2)].iter().cloned().collect();
}

#[test]
fn insert() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    assert_eq!(map.get("one"), Some(&1));
    assert_eq!(map.get("two"), Some(&2));
    assert_eq!(map.get("three"), Some(&3));
    assert_eq!(map.get("four"), None);

    assert_eq!(map["one"], 1);
    assert_eq!(map["two"], 2);
    assert_eq!(map["three"], 3);
}

#[test]
fn remove() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    assert_eq!(map.remove("two"), Some(2));
    assert_eq!(map.remove("two"), None);
}

#[test]
fn contains_key() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    if map.contains_key("one") {
        println!("one is in the map: {}", map["one"]);
    }

    if !map.contains_key("four") {
        println!("four is not in the map");
    }
}

#[test]
fn keys_and_values() {
    let mut map = HashMap::new();
    map.insert("apple", 1);
    map.insert("orange", 3);
    map.insert("kiwi", 2);

    for key in map.keys() {
        println!("key: {}", key);
    }

    let mut vs = map.keys().collect::<Vec<_>>();
    vs.sort_unstable();
    println!("{:?}", vs);

    for value in map.values() {
        println!("value: {}", value);
    }

    let mut vs = map.values().collect::<Vec<_>>();
    vs.sort_unstable();
    println!("{:?}", vs);
}

#[test]
fn get_mut_test() {
    let mut map = HashMap::new();
    map.insert("apple", 1);
    map.insert("orange", 3);
    map.insert("kiwi", 2);

    if let Some(value) = map.get_mut("apple") {
        *value += 10;
    }
    println!("{:?}", map);
}

#[test]
fn get_key_value_test() {
    let mut map = HashMap::new();
    map.insert("apple", 1);
    map.insert("orange", 3);
    map.insert("kiwi", 2);

    if let Some((key, value)) = map.get_key_value("apple") {
        println!("{}: {}", key, value);
    }
}

///
/// Entry API
///
#[test]
fn entry_or_insert() {
    let mut map = HashMap::new();

    map.entry("one").or_insert(1);
    map.entry("two").or_insert(1);
    map.entry("three").or_insert(1);

    *map.entry("four").or_insert(0) += 1;
    println!("{:?}", map);
}

#[test]
fn entry_or_default() {
    let mut map: HashMap<&str, u32> = HashMap::new();

    map.entry("one").or_default();
    map.entry("two").or_default();
    println!("{:?}", map);

    *map.entry("one").or_default() += 1;
    *map.entry("two").or_default() += 1;
    println!("{:?}", map);
}

#[test]
fn entry_modify_before_insert() {
    let mut map = HashMap::new();

    map.entry("one").and_modify(|v| *v += 1).or_default();
    map.entry("two").and_modify(|v| *v += 1).or_insert(0);
    println!("{:?}", map);

    map.entry("one").and_modify(|v| *v += 1).or_default();
    map.entry("two").and_modify(|v| *v += 1).or_insert(1);
    println!("{:?}", map);
}
