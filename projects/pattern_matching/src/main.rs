fn main() {
    // if let combos
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse::<u8>();

    if let Some(color) = favorite_color {
        println!("Using your favorit color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using bluee as the background color");
    }

    // while let conditional loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top_val) = stack.pop() {
        println!("{}", top_val);
    }

    // for loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // tuple destructuring
    // underscores remove compiler warnings
    let (_x, _y, _z) = (1, 2, 3);
    // This won't work though as the pattern doesn't match the expression
    // let (x, y) = (1, 2, 3);
    // Alternatively, can get past above issue with an _
    // let (x, y, _) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates((x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
