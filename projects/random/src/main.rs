use std::collections::HashSet;

fn main() {
    let s = String::from("hello world");
    let p = String::from("hello world!");

    let equality = s == p;
    println!("Equality: {}", equality);
    println!("s: {}\np: {}\n", s, p);

    let ref_equality = &s == &p;
    println!("Reference equality: {}", ref_equality);
    println!("&s: {}\n&p: {}", &s, &p);

    let s = String::from("abba");
    let char_vec: Vec<char> = s.chars().collect();
    println!("{:?}", char_vec);

    let mut v = vec![1, 2, 3, 1];
    let mut set = HashSet::new();
    v.retain(|e| set.insert(*e));
    println!("{:?}", v);
}
