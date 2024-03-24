#![allow(unused)]
trait Animal {
    fn name(&self) -> String;
    fn die() {
        println!("Oh no! I’m dead!");
    }
}

struct Dog;
impl Animal for Dog {
    fn name(&self) -> String {
        "Jindol".to_string()
    }
}

fn main() {
    let dog = Dog;
    println!("{:?}", dog.name());
    println!("{:?}", Animal::name(&dog));

    let name = <Dog as Animal>::name(&dog);
    println!("{name:?}");

    // must call via impl
    // Animal::die();
    <Dog as Animal>::die();
}
