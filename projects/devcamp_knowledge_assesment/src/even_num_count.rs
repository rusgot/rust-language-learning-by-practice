// Name: Jake Edwards
// Email: edwardsjake@live.com
// Just pasting the whole program for each answer as they are required fields

use std::io;

fn main() {
    println!("Please enter a random number: ");
    let mut user_input = String::new();

    // a) The user provides a number
    match io::stdin().read_line(&mut user_input) {
        Ok(_n) => {}
        Err(e) => panic!("Error: {}", e),
    }

    println!("You entered: {}", &user_input);

    // Convert String to i32
    let user_input_int: i32 = match user_input.trim().parse() {
        Ok(val) => val,
        Err(e) => panic!("Error: {}", e),
    };
    let count = even_num_count(user_input_int);
    println!(
        "Count of even numbers between 0 and {}(inclusive) is {}",
        user_input, count
    );
}

/// b) The program returns the count of all even numbers until that number, starting from 0
fn even_num_count(num: i32) -> i32 {
    let range: std::ops::Range<i32>;
    // c) You may assume that the user can only provide both positive and negative integers
    if num >= 0 {
        range = 0..num + 1
    } else {
        range = num..0 + 1;
    }
    // Say num is 5. So even nums are 0, 2, 4. Return 3
    let even_nums: Vec<i32> = range.filter(|n| n % 2 == 0).collect();
    println!("Even nums: {:?}, Count: {}", even_nums, even_nums.len());

    even_nums.len() as i32
}
