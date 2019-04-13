mod sound;

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod performance_group {
    pub use super::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

use menu::Appetizer;
use plant::Vegetable;

fn main() {
    performance_group::clarinet_trio();

    let mut v = Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    performance_group::instrument::clarinet();
}
