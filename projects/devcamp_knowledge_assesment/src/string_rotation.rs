// Name: Jake Edwards
// Email: edwardsjake@live.com
// Just pasting the whole program for each answer as they are required fields

use std::collections::HashMap;

fn main() {
    let s_1 = String::from("AXDFFG");
    let s_2 = String::from("FAGXDF");

    match string_rotation_check(s_1, s_2) {
        true => println!("TRUE, the strings are a rotation of each other!"),
        false => println!("FALSE, the strings are NOT a rotation of each other."),
    }
}

fn string_rotation_check(s_1: String, s_2: String) -> bool {
    // If lengths are different, `s_2` cannot be a rotation of `s_1`
    if s_1.len() != s_2.len() {
        return false;
    }

    // Convert both strings to char vectors for ease of use
    let v_1: Vec<char> = s_1.chars().collect();
    let v_2: Vec<char> = s_2.chars().collect();

    // Find a unique letter to start with in `s_1`
    let mut letter_count: HashMap<char, i32> = HashMap::new();
    for i in 0..v_1.len() {
        if letter_count.contains_key(&v_1[i]) {
            if let Some(x) = letter_count.get_mut(&v_1[i]) {
                *x = *x + 1;
            }
        } else {
            // Letter not encountered before
            letter_count.insert(v_1[i], 1);
            println!("{} inserted", v_1[i]);
        }
    }
    println!("{:?}", letter_count);

    // Pick 1st letter that has a count of 1
    let mut index_1: usize = 0;
    let mut index_2: usize = 0;

    for i in 0..v_1.len() {
        let count = letter_count.get(&v_1[i]).unwrap();
        if *count == 1 {
            index_1 = i;
            break;
        }
    }

    // Check is `s_2` contains the first letter of `s_1`
    if v_2.contains(&v_1[index_1]) {
        for i in 0..v_1.len() {
            if v_2[i] == v_1[index_1] {
                index_2 = i;
                break;
            }
        }
    } else {
        return false;
    }

    println!("index_1: {}", index_1);
    println!("index_2: {}", index_2);

    let index_difference: usize;
    if index_1 > index_2 {
        index_difference = index_1 - index_2;
    } else {
        index_difference = index_2 - index_1;
    }
    let index = |i| i % s_1.len();

    for i in 0..v_1.len() {
        let val_1 = v_1[i];
        let val_2 = v_2[index(i + index_difference)];

        if val_1 != val_2 {
            return false;
        }
    }
    true
}
