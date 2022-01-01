#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
}

// immutable borrow of a struct Rectangle instance
// This way `main` retains its ownership and can continue using rect1
fn area(rectangle: &Rectangle) -> u32 {
    // Debug prints to the console but expression still returns as normal
    dbg!(rectangle.width * rectangle.height)
}
