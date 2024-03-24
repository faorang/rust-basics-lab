use super::common::{Coord, Person, ToJson};
use serde::{Deserialize, Serialize};
use lib::delim;

#[cfg(feature = "skip")]
#[test]
fn person_to_json() {
    delim!();

    let person = Person::new("John", 42);

    println!("{}", person.to_json());
    assert_eq!(person.to_json(), r#"{"name":"John","age":42}"#);

    delim!();
}

#[cfg(feature = "skip")]
#[test]
fn coord_to_json() {
    delim!();

    let point = Coord { x: 1, y: 2 };

    println!("{}", point.to_json());
    assert_eq!(point.to_json(), r#"{"x":1,"y":2}"#);

    delim!();
}

#[test]
fn blank_implementation_for_references_for_car() {
    delim!();

    #[derive(Serialize, Deserialize, Debug)]
    struct Car {
        name: String,
        make: String,
        year: u32,
    }

    let mut car = Car {
        name: "Mustang".to_string(),
        make: "Ford".to_string(),
        year: 1969,
    };

    /*
     * If we implement `ToJson` trait for `Car` struct,
     * we can call `to_json` method on `car` instance, `&Car`, and `&mut Car`.
     * No need to define blanket implementations for `&T` and `&mut T`.
     */
    impl ToJson for Car {
        fn to_json(&self) -> String {
            println!("toJson for Car");
            serde_json::to_string(self).unwrap()
        }
    }

    println!("{}", car.to_json());
    delim!();

    // impl ToJson for &Car {
    //     fn to_json(&self) -> String {
    //         println!("toJson for &Car");
    //         serde_json::to_string(self).unwrap()
    //     }
    // }

    let car_ref = &car;
    println!("{}", car_ref.to_json());
    delim!();

    // impl ToJson for &mut Car {
    //     fn to_json(&self) -> String {
    //         println!("toJson for &mut Car");
    //         serde_json::to_string(self).unwrap()
    //     }
    // }

    let car_mut_ref = &mut car;
    println!("{}", car_mut_ref.to_json());
    delim!();
}

#[cfg(feature = "skip")]
#[test]
fn trait_bounds() {
    // Use trait bound
    fn print_json1<T: ToJson>(t: &T) {
        println!("{}", t.to_json());
    }

    // Use `where` clause
    fn print_json2<T>(t: &T)
    where
        T: ToJson,
    {
        println!("{}", t.to_json());
    }

    let person = Person::new("John", 42);
    print_json1(&person);

    let point = Coord::new(1, 2);
    print_json2(&point);
}
