#[allow(unused)]
use super::common::{Coord, Person, ToJson};
use lib::delim;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_as_single_parameter() {
        delim!();

        // Use `impl` as single parameter
        // fn print_json(t: !) {
        fn print_json(t: &impl ToJson) {
            println!("{}", t.to_json());
        }

        let person = Person::new("John", 42);
        // print_json(&person);

        let point = Coord::new(1, 2);
        // print_json(&point);

        delim!();
    }

    #[test]
    fn impls_as_parameters() {
        delim!();

        fn print_json1(t1: &impl ToJson, t2: &impl ToJson) {
            println!("{} {}", t1.to_json(), t2.to_json());
        }

        fn print_json2<T1: ToJson, T2: ToJson>(t1: &T1, t2: &T2) {
            println!("{} {}", t1.to_json(), t2.to_json());
        }

        fn print_json3<T: ToJson>(t1: &T, t2: &T) {
            println!("{} {}", t1.to_json(), t2.to_json());
        }

        let person = Person::new("John", 42);
        let point = Coord::new(1, 2);

        // print_json1(&person, &point);
        // print_json2(&person, &point);
        // print_json3(&person, &point);

        delim!();
    }

    #[test]
    fn impls_as_parameters_using_trait_object_behind_ref() {
        delim!();

        fn print_json(t: &dyn ToJson) {
            println!("{}", t.to_json());
        }

        let person = Person::new("John", 42);
        let point = Coord::new(1, 2);

        // print_json(&person);
        // print_json(&point);
        delim!();

        // let jsons: Vec<&dyn ToJson> = vec![&person, &point];
        // for json in jsons {
        //     print_json(json);
        // }

        delim!();
    }

    #[test]
    fn impls_as_parameters_using_trait_object_behind_box() {
        delim!();

        fn print_json(t: Box<dyn ToJson>) {
            println!("{}", t.to_json());
        }

        let person = Person::new("John", 42);
        let point = Coord::new(1, 2);

        // print_json(Box::new(person));
        // print_json(Box::new(point));
        delim!();

        let jsons: Vec<Box<dyn ToJson>> = vec![
            // Box::new(Person::new("John", 42)),
            // Box::new(Coord::new(1, 2)),
        ];

        for json in jsons {
            print_json(json);
        }

        delim!();
    }
}
