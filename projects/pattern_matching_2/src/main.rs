fn main() {
    // MATCHING LITERALS
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // MATCHING NAMED VARIABLES
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // `y` inside match scope is different than `y` declared outside of match scope
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);

    // MULTIPLE PATTERNS
    let x = 2;

    match x {
        1 | 2 => println!("One or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // MATCHING RANGES OF VALUES WITH ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("Something else"),
    }

    let x = 'k';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // DESTRUCTURING TO BREAK APART VALUES
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // shorthand syntax for above
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // DESTRUCTURING (NESTED) ENUMS
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // DESTRUCTUREING STRUCTS AND TUPLES
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    // IGNORING PARTS OF A VALUE WITH A NESTED _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("Setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // IGNORE AN UNUSED VARIABLE BY STARTING ITS NAME WITH _
    let _x = 5;

    // IGNORING REMAINING PARTS OF A VALUE WITH ..
    let origin = Point3d { x: 0, y: 0, z: 0 };

    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // ERROR CONDITIONALS WITH MATCH GUARDS
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ BINDINGS
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Message2 {
    Hello { id: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

// IGNORE PARAMETER(S)
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
