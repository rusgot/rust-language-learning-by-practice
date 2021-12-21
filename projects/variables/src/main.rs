fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // Shadowing
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

    // Floating-Point Numbers
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    // let sum = 5 + 10;

    // subtraction
    // let difference = 95.5 - 4.3;

    // multiplication
    // let product = 4 * 30;

    // division
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // Results in 0
    // let floored_float = 2.0 / 3.0; // Results in 0.6666666666666666

    // remainder
    // let remainder = 43 % 5;

    // Booleans
    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // Tuples
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {}", y);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("a first: {}", first);
    println!("b second: {}", b[1]);
}
