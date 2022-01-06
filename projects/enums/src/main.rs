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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("ayyyy 25 cents from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {
    println!("Rerolling...");
}
fn move_player(num_spaces: u8) {
    println!("Player moved {} spaces", num_spaces);
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let mover = Message::Move { x: 10, y: 12 };
    mover.call();

    let coin = Coin::Quarter(UsState::Texas);
    println!("{:?}", coin);
    let amount = value_in_cents(coin);
    println!("Amount: {}", amount);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Matching any remaining case with `other`
    // Variable for `other` is passed to code for arm
    let dice_roll = 12;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Use `_` when no need for matching value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // Unit value will do nothing when `dice_roll` does not match 3 or 7
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
