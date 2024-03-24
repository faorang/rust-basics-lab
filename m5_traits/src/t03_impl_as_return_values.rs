use super::common::{Coord, Person, ToJson};
use lib::delim;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "skip")]
    #[test]
    fn impl_as_a_return_values1() {
        delim!();

        fn get_json() -> impl ToJson {
            // Person::new("John", 42)
        }

        let person = &get_json();
        println!("{}", person.to_json());

        delim!();
    }

    #[cfg(feature = "skip")]
    #[test]
    fn impl_as_a_return_values2() {
        enum Kind {
            Person,
            Coord,
        }

        fn get_json(kind: Kind) -> impl ToJson {
            match kind {
                Kind::Person => Person::new("John", 42),
                Kind::Coord => Coord::new(1, 2),
            }
        }

        let person = get_json(Kind::Person);
    }

    #[test]
    fn homogeneous_collection() {
        delim!();

        let vs = vec![Coord { x: 1, y: 2 }, Coord { x: 3, y: 4 }];

        for v in vs {
            // println!("{}", v.to_json());
        }

        delim!();
    }

    #[cfg(feature = "skip")]
    #[test]
    fn using_trait_objects_references_by_casting_to_dyn() {
        delim!();

        let person = Person::new("John", 42);
        let point = Coord::new(1, 2);

        // let vs = vec![&person, &point];
        let vs = vec![&person as &dyn ToJson, &point as &dyn ToJson];

        for v in vs {
            println!("{}", v.to_json());
        }

        delim!();
    }

    #[cfg(feature = "skip")]
    #[test]
    fn using_explicitly_type_annotated_trait_objects_references() {
        delim!();

        let person = Person::new("John", 42);
        let point = Coord::new(1, 2);

        // let vs: Vec<_> = vec![&person, &point];
        let vs: Vec<&dyn ToJson> = vec![&person, &point];

        for v in vs {
            println!("{}", v.to_json());
        }

        delim!();
    }

    #[cfg(feature = "skip")]
    #[test]
    fn using_explicitly_type_annotated_boxed_trait_objects() {
        delim!();

        let person = Person::new("John", 42);
        let point = Coord::new(1, 2);

        // let vs: Vec<_> = vec![Box::new(person), Box::new(point)];
        let vs: Vec<Box<dyn ToJson>> = vec![Box::new(person), Box::new(point)];

        for v in vs {
            println!("{}", v.to_json());
        }

        delim!();
    }

    #[cfg(feature = "skip")]
    #[test]
    fn impl_as_a_return_value_solution_using_boxed_dyn() {
        delim!();

        enum Kind {
            Person,
            Coord,
        }

        fn get_json(kind: Kind) -> Box<dyn ToJson> {
            match kind {
                Kind::Person => Box::new(Person::new("John", 42)),
                Kind::Coord => Box::new(Coord::new(1, 2)),
            }
        }

        let value = get_json(Kind::Person);
        println!("{}", value.to_json());
        let value = get_json(Kind::Coord);
        println!("{}", value.to_json());

        delim!();
    }

    #[cfg(feature = "skip")]
    fn impl_as_a_return_value_wrong_solution() {
        enum Kind {
            Person,
            Coord,
        }

        fn get_json<'a>(kind: Kind) -> &'a dyn ToJson {
            match kind {
                Kind::Person => &Person::new("John", 42),
                Kind::Coord => &Coord::new(1, 2),
            }
        }

        let value = get_json(Kind::Person);
        println!("{}", value.to_json());
        let value = get_json(Kind::Coord);
        println!("{}", value.to_json());
    }
}
