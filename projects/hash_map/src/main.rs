use std::collections::HashMap;
fn main() {
    let mut state_codes = HashMap::new();
    state_codes.insert("KL", "Kerala");
    state_codes.insert("MH", "Maharashtra");
    println!("{:?}", state_codes);
    println!("Map size: {}", state_codes.len());

    match state_codes.get("KL") {
        Some(value) => {
            println!("Value for key KL is {}", value);
        }
        None => {
            println!("Nothing found");
        }
    }

    for (key, value) in state_codes.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    state_codes.insert("JE", "Jake Edwards");

    if state_codes.contains_key("JE") {
        println!("Key found");
    }

    let value = state_codes.get("JE");
    print_type_of(&value);

    match state_codes.get("J") {
        Some(value) => {
            println!("Value is {}", value);
        }
        None => {
            println!("Nothing returned");
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
