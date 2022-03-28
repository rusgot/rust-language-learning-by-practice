fn main() {
    let answer = do_twice(add_one, 6);

    println!("Answer is {}", answer);

    let list_of_statuses: Vec<Status> = (0u32..6).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);

    let stored_closure = returns_closure();
    // I think deref coercion is happening here
    println!("{}", stored_closure(1));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// Doesn't compile
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// Does compile
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
