mod jakes_module {
    use std::fmt;
    pub struct Person {
        pub name: String,
        age: u8,
        pub species: String,
    }

    impl Person {
        pub fn new_person(name: String, age: u8) -> Person {
            Person {
                name,
                age,
                species: String::from("Human"),
            }
        }
    }

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Name: {}, Age: {}, Species: {}",
                self.name, self.age, self.species
            )
        }
    }
}

fn create_jake() -> jakes_module::Person {
    let name = String::from("Jake Edwards");
    let age = 25;
    let jake = jakes_module::Person::new_person(name, age);
    jake
}

fn main() {
    let jake = create_jake();

    println!("{}", jake);
}
