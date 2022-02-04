fn main() {
    let x = String::from("hello from x outside");

    {
        let y = String::from("inner block y");
        let biggliest = longest(&x, &y);
        println!("Biggliest: {}", biggliest);
    }

    // This will not work because the HoldsStringRef struct only lives as long as its field with the
    // shortest lifetime. `z` is in the inner brackets so obj is only valid inside those brackets
    // despite being declared outside of the brackets.
    let obj;
    {
        let z = String::from("hello");
        obj = HoldsStringRef { s_slice: &z };
    }
    println!("obj: {:?}", obj.s_slice);
}

/// The returned reference will be valid for the length of the smaller of the lifetimes of `x` and `y`
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct HoldsStringRef<'a> {
    s_slice: &'a str,
}
