fn main() {
    let mut s1 = String::from("Hello ");
    let s2 = "world";

    // Append `s2` to `s1`
    s1.push_str(s2);
    println!("{}", s1);

    // Append single character to `s1`
    // char uses single quotes '' not double quotes ""
    s1.push('!');
    println!("{}", s1);

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    // format! macro returns the formatted String
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // String slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Iterating over Strings
    for c in "hello".chars() {
        println!("Char: {}", c);
    }

    // Get bytes from String
    for c in "hello".bytes() {
        println!("Byte: {}", c);
    }
}
