// References are immutable by default. Below code does not work!
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// Fixing immutable reference issue in code above
// change() function now takes a mutable reference
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{}", s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// You can have only one mutable reference to a particular piece of data at a time
// Code will fail
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }
