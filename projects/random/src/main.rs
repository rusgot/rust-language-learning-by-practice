fn main() {
    let s = String::from("hello world");
    let p = String::from("hello world!");

    let equality = s == p;
    println!("Equality: {}", equality);
    println!("s: {}\np: {}\n", s, p);

    let ref_equality = &s == &p;
    println!("Reference equality: {}", ref_equality);
    println!("&s: {}\n&p: {}", &s, &p);
}
