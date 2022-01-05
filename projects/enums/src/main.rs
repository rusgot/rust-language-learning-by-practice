enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("hello world");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let mover = Message::Move { x: 10, y: 12 };
    mover.call();
}
